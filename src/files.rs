use std::path::Path;

use std::fs;
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub files_dir: String,
    pub model_path: String,
    pub model_url: String,
    pub scorer_path: String,
    pub scorer_url: String,
    pub output_dir: String,
    pub s3_bucket: String,
    pub audio_src: String,
    pub shortname: String,
    pub local_file: String,
}

pub fn load_config() -> Config {
    let loaded_str = fs::read_to_string("config.json").unwrap();
    let config_json: Config = serde_json::from_str(&loaded_str).unwrap();
    let shortname = get_shortname(&config_json.audio_src);
    let local_file = create_dest_path(&config_json);

    Config {
        shortname,
        local_file,
        ..config_json
    }
}

pub fn create_dest_path(config: &Config) -> String {
    let filename = get_filename_from_url(&config.audio_src).expect("Cant get filename from url");
    let dest_path = format!("{}/{}", config.files_dir, filename);
    dest_path
}

pub fn get_filename_from_url(audio_src: &str) -> Option<String> {
    let path = Path::new(audio_src);
    match path.file_name() {
        Some(os_str) => match os_str.to_str() {
            Some(s) => Some(s.to_string()),
            None => None,
        },
        None => None,
    }
}

pub async fn download_bytes(url: &str, save_path: &str) -> anyhow::Result<()> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    fs::write(save_path, bytes)?;
    Ok(())
}

pub fn get_shortname(audio_src: &str) -> String {
    let src_file = Path::new(audio_src);
    let stripped_ext = src_file.with_extension("");
    let raw_filename = stripped_ext.file_name().unwrap().to_str().unwrap();
    String::from(raw_filename)
}

#[allow(dead_code)]
fn is_valid_url(input: &str) -> bool {
    match url::Url::parse(input) {
        Ok(_) => true,
        Err(_) => false,
    }
}
