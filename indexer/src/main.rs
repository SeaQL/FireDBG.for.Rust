use anyhow::{Context, Result};
use flume::bounded;
use sea_streamer::{
    file::{FileErr, FileId},
    runtime::{sleep, spawn_task},
    BackendErr, Buffer, Consumer, ConsumerMode, ConsumerOptions, Message, SeaConsumerOptions,
    SeaMessage, SeaStreamReset, SeaStreamer, SharedMessage, StreamErr, StreamKey, Streamer,
};
use std::collections::HashMap;
use structopt::StructOpt;

use firedbg_rust_debugger::{
    Event, EventStream, ALLOCATION_STREAM, BREAKPOINT_STREAM, EVENT_STREAM, FILE_STREAM,
    INFO_STREAM,
};
use firedbg_stream_indexer::{
    database::{
        insert_allocations, insert_breakpoints, insert_events, insert_files, insert_type_info,
        save_debugger_info, Database,
    },
    translate,
    util::{deser, deser_info},
    validator::Validator,
    Processor,
};

const TEMPLATE: &str = concat!(
    "{bin} {version}\n",
    "  by SeaQL.org

USAGE:
    {usage}

{all-args}

AUTHORS:
    {author}
"
);

#[derive(StructOpt, Debug)]
#[structopt(
    template = TEMPLATE,
    author,
)]
struct Args {
    #[structopt(long, help = "Input .firedbg.ss file")]
    input: FileId,
    #[structopt(long, help = "Output .sqlite file", default_value = "output.sqlite")]
    output: String,
    #[structopt(subcommand)]
    sub_command: Option<SubCommand>,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    /// Validate the content of a .firedbg.ss file against a JSON file
    Validate {
        #[structopt(long, help = "The JSON file that contains the expected content")]
        json: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let Args {
        input,
        output,
        sub_command,
    } = Args::from_args();

    match sub_command {
        Some(SubCommand::Validate { json }) => {
            let validator = Validator::new(json);
            run(input, validator).await?;
        }
        None => {
            let sink = DatabaseSink {
                db: Database::create(output).await?,
                stack: Default::default(),
                count: 0,
            };
            run(input, sink).await?;
        }
    }

    Ok(())
}

async fn run<P: Processor>(input: FileId, mut processor: P) -> Result<()> {
    let streamer = SeaStreamer::connect(input.to_streamer_uri()?, Default::default()).await?;
    let info_stream = StreamKey::new(INFO_STREAM)?;
    let file_stream = StreamKey::new(FILE_STREAM)?;
    let event_stream = StreamKey::new(EVENT_STREAM)?;
    let breakpoint_stream = StreamKey::new(BREAKPOINT_STREAM)?;
    let alloc_stream = StreamKey::new(ALLOCATION_STREAM)?;

    let mut options = SeaConsumerOptions::new(ConsumerMode::RealTime);
    options.set_auto_stream_reset(SeaStreamReset::Earliest);

    let stream_keys = [
        info_stream,
        file_stream,
        event_stream,
        breakpoint_stream,
        alloc_stream,
    ];
    let consumer = streamer.create_consumer(&stream_keys, options).await?;

    // From sea-streamer/examples/buffered
    let (sender, receiver) = bounded(1024);

    let handle = spawn_task::<_, Result<(), StreamErr<BackendErr>>>(async move {
        loop {
            let message = consumer.next().await?;
            let err = |_| StreamErr::Backend(BackendErr::File(FileErr::TaskDead("channel closed")));
            // If the queue is full, we'll wait
            sender
                .send_async(unpack(message))
                .await
                .context("Fail to buffer message")
                .map_err(err)?;
        }
    });

    while !receiver.is_empty() || !receiver.is_disconnected() {
        // Take all messages currently buffered in the queue, but do not wait
        let messages = receiver.drain();
        if messages.len() > 1 {
            processor.batch(messages).await?;
        } else if messages.len() == 1 {
            // if there is only 1 item; wait for more
            sleep(std::time::Duration::from_millis(1)).await;
            processor.batch(messages.chain(receiver.drain())).await?;
        } else {
            // no messages; sleep
            sleep(std::time::Duration::from_millis(10)).await;
        }
    }

    processor.end().await?;
    if let Err(e) = handle.await? {
        ok_if_stream_ended(e)?;
    }

    processor.finish()
}

struct DatabaseSink {
    db: Database,
    /// thread id -> frame id[]
    stack: HashMap<u64, Vec<u64>>,
    count: usize,
}

#[async_trait::async_trait]
impl Processor for DatabaseSink {
    async fn batch(&mut self, messages: impl Iterator<Item = SharedMessage> + Send) -> Result<()> {
        let mut files = Vec::new();
        let mut bps = Vec::new();
        let mut events = Vec::new();
        let mut types = Vec::new();
        let mut allocs = Vec::new();
        let mut flush = false;

        for message in messages {
            match message.header().stream_key().name() {
                INFO_STREAM => {
                    save_debugger_info(&self.db, translate::debugger_info(deser_info(&message)))
                        .await?
                }
                FILE_STREAM => files.push(deser(&message)),
                BREAKPOINT_STREAM => bps.push(deser(&message)),
                EVENT_STREAM => events.push({
                    let event = EventStream::read_from(message.message().into_bytes().into());
                    translate::type_info(&event, |ty| types.push(ty));
                    let mut parent_frame_id = None;
                    match &event {
                        Event::FunctionCall {
                            thread_id,
                            frame_id,
                            ..
                        } => {
                            let stack = self.stack.entry(*thread_id).or_default();
                            parent_frame_id = stack.last().copied();
                            stack.push(*frame_id);
                        }
                        Event::FunctionReturn {
                            thread_id,
                            frame_id,
                            ..
                        } => {
                            assert_eq!(
                                *frame_id,
                                self.stack
                                    .get_mut(thread_id)
                                    .expect("Thread not found")
                                    .pop()
                                    .expect("Stack frame empty")
                            );
                        }
                        _ => (),
                    }
                    let mut event = translate::event(message.timestamp(), event);
                    event.parent_frame_id = sea_orm::Set(parent_frame_id.map(|s| s as i64));
                    event
                }),
                ALLOCATION_STREAM => allocs.push(deser(&message)),
                _ => anyhow::bail!("Unexpected stream key {}", message.stream_key()),
            }
            self.count += 1;
            flush |= self.count % 10000 == 0;
        }

        insert_files(&self.db, files.into_iter().map(translate::source_file)).await?;
        insert_breakpoints(&self.db, bps.into_iter().map(translate::breakpoint)).await?;
        insert_events(&self.db, events.into_iter()).await?;
        insert_type_info(&self.db, types.into_iter()).await?;
        insert_allocations(&self.db, allocs.into_iter().map(translate::allocation)).await?;

        if flush {
            // this flushes the WAL and makes the data queryable
            self.db.reopen().await?;
        }

        Ok(())
    }

    async fn end(&mut self) -> Result<()> {
        self.db.close().await?;
        Ok(())
    }

    fn finish(self) -> Result<()> {
        Ok(())
    }
}

fn ok_if_stream_ended(e: StreamErr<BackendErr>) -> Result<()> {
    match e {
        StreamErr::Backend(BackendErr::File(FileErr::StreamEnded)) => Ok(()),
        _ => Err(e)?,
    }
}

fn unpack(message: SeaMessage<'_>) -> SharedMessage {
    match message {
        SeaMessage::File(m) => m,
        _ => panic!("Expected FileMessage"),
    }
}
