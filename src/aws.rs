#![allow(unused)]

use std::path::Path;
use std::process;
use clap::Parser;

use aws_sdk_s3::{ByteStream, Client, Error, output::PutObjectOutput};

use crate::files::{Config, Args};

pub async fn s3_upload(config: &Config, path: &str) -> anyhow::Result<()> {
    let args = Args::parse();

    let aws_config = aws_config::load_from_env().await;
    let client = Client::new(&aws_config);
    let file = ByteStream::from_path(Path::new(path)).await;
    let resp;
    println!("Uploading...");
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
    println!("Uploaded!");
    Ok(())
}
