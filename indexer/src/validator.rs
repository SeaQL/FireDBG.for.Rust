use anyhow::{Context, Result};
use async_trait::async_trait;
use sea_streamer::{Buffer, Message, SharedMessage};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::{
    util::{deser, deser_info},
    Processor,
};
use firedbg_rust_debugger::{
    Breakpoint, Event, EventStream, InfoMessage, SourceFile, BREAKPOINT_STREAM, EVENT_STREAM,
    FILE_STREAM, INFO_STREAM,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Validator {
    json: String,
    data: ValidatorData,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValidatorData {
    pub debugger_infos: Vec<InfoMessage>,
    pub files: Vec<SourceFile>,
    pub breakpoints: Vec<Breakpoint>,
    pub events: Vec<Event>,
}

impl Validator {
    pub fn new(json: String) -> Self {
        Self {
            json,
            data: Default::default(),
        }
    }
}

#[async_trait]
impl Processor for Validator {
    async fn batch(&mut self, messages: impl Iterator<Item = SharedMessage> + Send) -> Result<()> {
        for message in messages {
            match message.header().stream_key().name() {
                INFO_STREAM => self.data.debugger_infos.push({
                    let mut debugger_info = deser_info(&message);
                    debugger_info.redacted();
                    debugger_info
                }),
                FILE_STREAM => self.data.files.push({
                    let mut file: SourceFile = deser(&message);
                    file.redacted();
                    file
                }),
                BREAKPOINT_STREAM => self.data.breakpoints.push(deser(&message)),
                EVENT_STREAM => {
                    let mut event = EventStream::read_from(message.message().into_bytes().into());
                    event.redacted();
                    self.data.events.push(event);
                }
                _ => anyhow::bail!("Unexpected stream key {}", message.stream_key()),
            }
        }
        Ok(())
    }

    async fn end(&mut self) -> Result<()> {
        Ok(())
    }

    fn finish(self) -> Result<()> {
        // println!("{}", serde_json::to_string_pretty(&self.data.events)?); std::process::exit(1);

        let expected = fs::read_to_string(&self.json)
            .with_context(|| format!("File not found: `{}`", self.json))?;
        let expected: Vec<Event> = serde_json::from_str(&expected)?;

        pretty_assertions::assert_eq!(expected, self.data.events);

        Ok(())
    }
}
