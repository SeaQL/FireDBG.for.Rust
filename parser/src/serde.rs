use anyhow::{Context, Result};
use sea_streamer_file::ByteSource;

pub async fn to_bson_file<S>(path: &str, bson: &S) -> Result<()>
where
    S: serde::Serialize,
{
    let mut file_sink = create_file_sink(path)
        .await
        .context("Fail to create file sink")?;
    bson::to_document(bson)?
        .to_writer(file_sink.as_writer())
        .context("Fail to write BSON")?;
    file_sink.flush(1).await.context("Fail to flush")?;
    Ok(())
}

pub async fn to_json_file<S>(path: &str, json: &S) -> Result<()>
where
    S: serde::Serialize,
{
    let mut file_sink = create_file_sink(path)
        .await
        .context("Fail to create file sink")?;
    serde_json::to_writer_pretty(file_sink.as_writer(), json).context("Fail to write JSON")?;
    file_sink.flush(1).await.context("Fail to flush")?;
    Ok(())
}

async fn create_file_sink<T>(path: T) -> Result<sea_streamer_file::FileSink>
where
    T: Into<String>,
{
    let file_id = sea_streamer_file::FileId::new(path);
    let async_file = sea_streamer_file::AsyncFile::new_ow(file_id)
        .await
        .context("Fail to create async file")?;
    let res = sea_streamer_file::FileSink::new(async_file, 1_000_000_000)
        .context("Fail to create file sink")?;
    Ok(res)
}

pub async fn from_bson_file<T>(path: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let file_id = sea_streamer_file::FileId::new(path);
    let mut file_reader = sea_streamer_file::FileReader::new(file_id)
        .await
        .context("Fail to create file reader")?;
    let file_size = file_reader.file_size();
    let bytes = file_reader
        .request_bytes(file_size as usize)
        .await
        .context("Fail to read")?;
    let slice = &bytes.bytes();
    let res = bson::de::from_slice(slice).context("Fail to deserialize BSON")?;
    Ok(res)
}
