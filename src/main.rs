#![allow(unused)]
mod aws;
mod constants;
mod fetchers;
mod ffmpeg;
mod files;
mod stt;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    env,
    time::{Duration, Instant},
};
use stt::Transcripts;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessedFile {
    pub stt_data: Transcripts,
    pub src_file: String,
    pub dest_file: String,
    pub shortname: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    aws::
    // let start_time = Instant::now();

    // let pb = ProgressBar::new_spinner();
    // pb.enable_steady_tick(Duration::from_millis(300));
    // pb.set_style(
    //     ProgressStyle::default_spinner()
    //         .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
    //         .template("{spinner:.green} [{elapsed:.yellow}] {msg:.blue}")?,
    // );
    // pb.set_message(format!("Processing audio files..."));

    // let src_file = env::args().nth(1).expect("No audio file provided");
    // ffmpeg::process_mp3s(&src_file)?;

    // let ffmpeg_end = Instant::now();
    // let ffmpeg_elapsed = ffmpeg_end.duration_since(start_time);
    // let ffmpeg_total = ffmpeg_elapsed.as_secs();

    // pb.set_message("Running stt processing");

    // stt::process_wav_segments(&src_file)?;

    // let stt_end = Instant::now();
    // let stt_elapsed = stt_end.duration_since(start_time);
    // let stt_total = stt_elapsed.as_secs();

    // pb.finish_with_message(format!(
    //     "\n\nCompleted wav conversion in: {} seconds\nCompleted stt conversion in: {}",
    //     ffmpeg_total, stt_total
    // ));
    Ok(())
}
