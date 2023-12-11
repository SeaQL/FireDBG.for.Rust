use anyhow::Result;
use async_trait::async_trait;
use sea_streamer::SharedMessage;

#[async_trait]
pub trait Processor {
    async fn batch(&mut self, messages: impl Iterator<Item = SharedMessage> + Send) -> Result<()>;

    async fn end(&mut self) -> Result<()>;

    fn finish(self) -> Result<()>;
}
