use crate::files::{Config, Args};
use indicatif::ProgressBar;
use rayon::prelude::*;
use clap::Parser;

use std::{
    fs::{self},
    process::{Command, Stdio},
};

pub fn process_mp3s(pb: &ProgressBar, config: &Config) -> anyhow::Result<()> {
    let cli_args = Args::parse();
    let contains_only = cli_args.only.contains(&String::from("ffmpeg"));
    let only_is_empty = cli_args.only.is_empty();

    if !only_is_empty && contains_only {
        pb.set_message("Converting to wav...");
        process_segments(config)?;
        return Ok(());
    }
    println!("skipping ffmpeg");
    Ok(())
}

fn process_segments(config: &Config) -> anyhow::Result<()> {
    let segments = calc_segments(&config.local_file);
    let audio_dir = format!("{}/wav/{}", &config.output_dir, &config.shortname);

    fs::create_dir_all(audio_dir).expect("Failed to create audio directory");

    segments.par_iter().for_each(|(start, end)| {
        let c = config.clone();
        let output_file = format!(
            "{}/wav/{}/{}_{}-{}.wav",
            c.output_dir, c.shortname, c.shortname, start, end
        );

        let command = format!(
            "ffmpeg -i {} -ss {} -to {} -ar 16000 -ac 1 -acodec pcm_s16le -f wav {}",
            &c.local_file, start, end, output_file
        );

        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command");
    });
    Ok(())
}

fn calc_segments(src_path: &str) -> Vec<(i32, i32)> {
    let segment_len = 600;
    let ffprobe_args = vec![
        "-v",
        "error",
        "-show_entries",
        "format=duration",
        "-of",
        "default=noprint_wrappers=1:nokey=1",
        src_path,
    ];

    let out = Command::new("ffprobe")
        .args(ffprobe_args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn ffprobe")
        .wait_with_output()
        .expect("Failed to wait for ffprobe");

    let s = String::from_utf8(out.stdout).expect("Failed to parse output as string");
    let total_len = s
        .trim()
        .parse::<f64>()
        .expect("Failed to parse output as float");

    let num_segments = (total_len / segment_len as f64).ceil() as i32;

    let segments: Vec<(i32, i32)> = (0..num_segments)
        .into_par_iter()
        .map(|i| (i * segment_len, (i + 1) * segment_len))
        .collect();

    segments
}
