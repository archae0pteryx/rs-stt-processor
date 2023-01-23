#![allow(unused)]

use std::path::Path;
use std::process;

use aws_sdk_s3::{ByteStream, Client, Error, output::PutObjectOutput};

pub async fn s3_upload(path: &str) -> anyhow::Result<PutObjectOutput> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let file = ByteStream::from_path(Path::new(path)).await;
    let resp;
    match file {
        Ok(f) => {
            resp = client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(f)
                .send()
                .await?;
        }
        Err(e) => {
            panic!("Error uploading file: {:?}", e);
        }
    };

    Ok(resp)
}
