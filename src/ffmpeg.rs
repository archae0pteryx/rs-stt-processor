use crate::{constants::SPLIT_AUDIO_DIR, files};
use rayon::prelude::*;
use std::{
    fs::{self},
    process::{Command, Stdio},
};

pub fn process_mp3s(src_file: &str) -> anyhow::Result<()> {
    process_segments(src_file)?;
    Ok(())
}

fn process_segments(src_file: &str) -> anyhow::Result<()> {
    let segments = calc_segments(src_file);

    let shortname = files::get_shortname(src_file).clone();
    let audio_dir = format!("{}/{}", SPLIT_AUDIO_DIR.clone(), shortname);

    fs::create_dir_all(audio_dir).expect("Failed to create audio directory");

    segments.par_iter().for_each(|(start, end)| {
        let output_file = format!(
            "{}/{}/{}_{}-{}",
            SPLIT_AUDIO_DIR, shortname, shortname, start, end
        );

        let command = format!(
            "ffmpeg -i {} -ss {} -to {} -ar 16000 -ac 1 -acodec pcm_s16le -f wav {}",
            src_file, start, end, output_file
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
