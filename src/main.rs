#![allow(unused)]
mod aws;
mod ffmpeg;
mod files;
mod stt;
mod waveform;
mod youtube_dl;
mod state;

use anyhow::Result;
use ffmpeg::split_audio;
use state::AppState;
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

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let state = AppState::new("https://www.youtube.com/watch?v=xZIsuR6VVKE");
    // youtube_dl(state);
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
