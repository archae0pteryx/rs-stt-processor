#![allow(unused)]
mod aws;
mod ffmpeg;
mod files;
mod stt;
mod waveform;
mod youtube_dl;
use anyhow::Result;
use ffmpeg::split_audio;
use std::process::{Command, Stdio};
use youtube_dl::youtube_dl;

use indicatif::{ProgressBar, ProgressStyle};
use std::{
    path::Path,
    time::{Duration, Instant},
};

use env_logger;
use std::env;
use log;

pub const WORKDIR: &str = "output";

#[derive(Clone)]
pub struct Target {
    url: String,
    raw_path: String,
    raw_file_path: String,
    wav_path: String,
    name: String,
}

impl Target {
    pub fn new(url: &str) -> Self {
        let name = url.split("=").last().unwrap();
        let root_path = String::from(WORKDIR);
        let raw_path = format!("{}/raw", root_path);
        let wav_path = format!("{}/wav", root_path);
        Target {
            url: url.to_string(),
            name: name.to_string(),
            raw_path: raw_path.clone(),
            wav_path,
            raw_file_path: format!("{}/{}.wav", raw_path, name),
        }
    }

    pub async fn download_youtube(mut self) -> Self {
        if !Path::new(&self.raw_file_path).exists() {
            self.raw_file_path = youtube_dl(&self.url).await;
        } else {
            dbg!("file already exists. skipping download");
        }
        self
    }

    pub fn split_wav(mut self) -> Self {
        let cp_self = self.clone();
        let dest = create_dest_dir(cp_self);
        split_audio(&self.raw_file_path.as_str(), dest.as_str());
        self
    }
}

fn create_dest_dir(target: Target) -> String {
    let dest_dir = format!("{}/{}", target.wav_path, target.name);
    if !Path::new(&dest_dir).exists() {
        std::fs::create_dir_all(&dest_dir).expect("Failed to create audio directory");
    }
    dest_dir
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    Target::new("https://www.youtube.com/watch?v=xZIsuR6VVKE")
        .download_youtube()
        .await
        .split_wav();
    // let config = files::load_config();
    // let start_time = Instant::now();

    // let pb = create_pb();

    // if Path::new(&config.local_file).exists() {
    //     pb.set_message("Audio file already exists, skipping download...");
    // } else {
    //     pb.set_message("Downloading audio file...");
    //     files::download_bytes(&config.audio_src, &config.local_file).await?;
    // }

    // ffmpeg::process_mp3s(&pb, &config)?;

    // let ffmpeg_end = Instant::now();
    // let ffmpeg_elapsed = ffmpeg_end.duration_since(start_time);
    // let ffmpeg_total = ffmpeg_elapsed.as_secs();

    // stt::process_stt(&pb, &config).await?;

    // waveform::gen_waveform(&pb, &config).await?;

    // let stt_end = Instant::now();
    // let stt_elapsed = stt_end.duration_since(start_time);
    // let stt_total = stt_elapsed.as_secs();

    // pb.finish_with_message(format!(
    //     "\n\nCompleted wav conversion in: {} seconds\nCompleted stt conversion in: {}",
    //     ffmpeg_total, stt_total
    // ));
    Ok(())
}

fn create_pb() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(300));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.green} [{elapsed:.yellow}] {msg:.cyan}")
            .unwrap(),
    );
    pb
}
