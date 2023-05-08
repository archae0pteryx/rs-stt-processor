#![allow(unused)]
mod aws;
mod ffmpeg;
mod files;
mod stt;
mod waveform;

use anyhow::Result;
use files::Config;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs,
    path::Path,
    time::{Duration, Instant},
};
use vosk::{Model, Recognizer};

use rodio::source::SamplesConverter;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;

#[tokio::main]
async fn main() -> Result<()> {
    let config = files::load_config();

    let file = File::open("800/sample-800.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let samples = source.convert_samples::<i16>().collect::<Vec<_>>();
    println!("Loaded {} samples", samples.len());

    let model_path = "models/vosk-model-en-us-0.42-gigaspeech";

    let model = Model::new(model_path).unwrap();
    let mut recognizer = Recognizer::new(&model, 16000.0).unwrap();

    recognizer.set_max_alternatives(10);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    for sample in samples.chunks(100) {
        recognizer.accept_waveform(sample);
        println!("{:#?}", recognizer.partial_result());
    }

    // println!("{:#?}", recognizer.final_result().multiple().unwrap());
    // let start_time = Instant::now();

    // files::cleanup_files(&config);

    // let pb = create_pb();

    // if files::check_if_local_file_exists(&config) {
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
