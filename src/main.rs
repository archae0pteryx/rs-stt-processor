// #![allow(unused)]
mod aws;
mod ffmpeg;
mod files;
mod stt;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::{time::{Duration, Instant}, path::Path};

#[tokio::main]
async fn main() -> Result<()> {
    let config = files::load_config();
    let start_time = Instant::now();

    let pb = create_pb();
    pb.set_message("Downloading audio file...");

    if Path::new(&config.local_file).exists() {
        pb.set_message("Audio file already exists, skipping download...");
    } else {
        files::download_bytes(&config.audio_src, &config.local_file).await?;
    }

    pb.set_message("Converting to wav...");

    ffmpeg::process_mp3s(&config)?;

    let ffmpeg_end = Instant::now();
    let ffmpeg_elapsed = ffmpeg_end.duration_since(start_time);
    let ffmpeg_total = ffmpeg_elapsed.as_secs();

    pb.set_message("Running stt processing...");

    stt::process_wav_segments(&config)?;

    let stt_end = Instant::now();
    let stt_elapsed = stt_end.duration_since(start_time);
    let stt_total = stt_elapsed.as_secs();

    pb.finish_with_message(format!(
        "\n\nCompleted wav conversion in: {} seconds\nCompleted stt conversion in: {}",
        ffmpeg_total, stt_total
    ));
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
