#![allow(unused)]

use std::path::Path;
use std::process;

use aws_sdk_s3::{ByteStream, Client, Error, output::PutObjectOutput};

use crate::files::Config;

pub async fn s3_upload(config: &Config, path: &str) -> anyhow::Result<PutObjectOutput> {
    let aws_config = aws_config::load_from_env().await;
    let client = Client::new(&aws_config);
    let file = ByteStream::from_path(Path::new(path)).await;
    let resp;
    println!("Uploading file to bucket: {}", path);
    match file {
        Ok(f) => {
            resp = client
                .put_object()
                .bucket(&config.s3_bucket)
                .key(path)
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
