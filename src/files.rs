use std::path::Path;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;

use clap::Parser;

#[derive(Parser, Debug, Serialize, Deserialize, Default)]
pub struct Args {
    pub src: Option<String>,

    #[clap(long)]
    pub only: Vec<String>,

    #[arg(default_value = "")]
    #[clap(short, long)]
    pub out: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub files_dir: String,
    pub model_path: String,
    pub model_url: String,
    pub scorer_path: String,
    pub scorer_url: String,
    pub output_dir: String,
    pub s3_bucket: String,
    #[serde(default = "String::new")]
    pub audio_src: String,
    #[serde(default = "String::new")]
    pub shortname: String,
    #[serde(default = "String::new")]
    pub local_file: String,
    #[serde(default = "String::new")]
    pub wavs_dir: String,
    #[serde(default = "String::new")]
    pub ep_output_dir: String,
    pub wav_chunk_size: i32,
}

pub fn load_config() -> Config {
    let args = Args::parse();
    let loaded_str = fs::read_to_string("config.json").unwrap();
    let config_json: Config = serde_json::from_str(&loaded_str).unwrap();
    let local_file = create_dest_path(&config_json);

    let src_to_use = get_src_to_use(&args, &config_json.audio_src);

    let shortname = get_shortname(&src_to_use);
    let ep_output_dir = format!("{}/{}", config_json.output_dir, shortname);

    let c = Config {
        shortname: shortname.to_owned(),
        local_file,
        audio_src: src_to_use,
        wavs_dir: format!("{}/wav", &config_json.output_dir),
        ep_output_dir,
        ..config_json
    };
    dbg!(&c);
    c
}

pub fn cleanup_files(config: &Config) {
    cleanup_ep_output_dir(config);
    cleanup_wavs(config);
}

fn get_src_to_use(args: &Args, cfg_src: &str) -> String {
    args.src.as_ref().map(|s| s.to_string()).unwrap_or_else(|| {
        if !cfg_src.is_empty() {
            cfg_src.to_string()
        } else {
            panic!("No audio source provided");
        }
    })
}

fn create_dest_path(config: &Config) -> String {
    let filename = get_filename_from_url(&config.audio_src).expect("Cant get filename from url");
    let dest_path = format!("{}/{}", config.files_dir, filename);
    dest_path
}

pub fn check_if_local_file_exists(config: &Config) -> bool {
    let check = get_filename_from_url(config.audio_src.as_str());
    match check {
        Some(filename) => {
            let local_file = format!("{}/{}", config.files_dir, filename);
            Path::new(&local_file).exists()
        }
        None => false,
    }
}

fn get_filename_from_url(audio_src: &str) -> Option<String> {
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

fn get_shortname(audio_src: &str) -> String {
    let src_file = Path::new(audio_src);
    let stripped_ext = src_file.with_extension("");
    let raw_filename = stripped_ext.file_name().unwrap().to_str().unwrap();
    String::from(raw_filename)
}

fn cleanup_wavs(config: &Config) {
    let folder_path = Path::new(config.wavs_dir.as_str());
    if folder_path.exists() {
        fs::remove_dir_all(folder_path).unwrap();
    }
}

fn cleanup_ep_output_dir(config: &Config) {
    let folder_path = Path::new(config.ep_output_dir.as_str());
    if folder_path.exists() {
        fs::remove_dir_all(folder_path).unwrap();
    }
}
