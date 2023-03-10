use crate::aws;
use crate::files::{Args, Config};
use clap::Parser;
use indicatif::ProgressBar;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::Barrier;
use std::sync::{Arc, Mutex};
use std::{fs::File, path::Path};
use walkdir::WalkDir;

fn stt_file_path(config: &Config) -> String {
    format!("{}/json/{}.stt.json", &config.output_dir, &config.shortname)
}

pub async fn process_stt(pb: &ProgressBar, config: &Config) -> anyhow::Result<()> {
    let cli_args = Args::parse();
    let contains_only = cli_args.only.contains(&String::from("stt"));
    let contains_upload = cli_args.upload.contains(&String::from("stt"));
    let only_is_empty = cli_args.only.is_empty();
    let upload_is_empty = cli_args.upload.is_empty();

    if (only_is_empty && upload_is_empty) || (!contains_upload && contains_only) {
        pb.set_message("Running stt processing...");
        process_wav_segments(&config).await?;
    } else {
        println!("skipping stt processing");
    }

    if (upload_is_empty && only_is_empty) || (!contains_only && contains_upload) {
        pb.set_message("Uploading stt to s3");
        let output_file = stt_file_path(&config);
        aws::s3_upload(&config, &output_file).await?;
    } else {
        println!("skipping stt upload");
    }

    Ok(())
}

async fn process_wav_segments(config: &Config) -> anyhow::Result<()> {
    let out_json_path = stt_file_path(&config);

    if Path::new(&out_json_path).exists() {
        println!("{} already exists, skipping...", out_json_path);
        return Ok(());
    }

    validate_models_exist(&config);

    let wav_segment_paths = get_wav_segment_paths(&config);

    let barrier = Barrier::new(wav_segment_paths.len());
    let json_results = Arc::new(Mutex::new(vec![]));

    wav_segment_paths.par_iter().for_each(|wav_path| {
        let stt_data = exec_stt(&config, wav_path);
        let word_data = extract_stt_word_data(&stt_data);
        json_results.lock().unwrap().extend(word_data);
        barrier.wait();
    });

    let json_results = json_results.clone();
    let mut json_results = json_results.lock().unwrap();
    json_results.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());

    let j = serde_json::to_string(&*json_results)
        .expect("Failed to convert mutex word vec to json string");

    fs::create_dir_all(format!("{}/json", &config.output_dir))
        .expect("Failed to create json directory");
    let mut json_file = File::create(&out_json_path)
        .expect(format!("Error creating json file: {}", out_json_path).as_str());
    json_file
        .write_all(j.as_bytes())
        .expect("Failed to write json file");

    Ok(())
}

fn exec_stt(config: &Config, wav_path: &str) -> String {
    let model_file = config.model_path.as_str();
    let scorer_file = config.scorer_path.as_str();

    let stt_args = vec![
        "--model",
        model_file,
        "--scorer",
        scorer_file,
        "--audio",
        wav_path,
        "--json",
    ];

    let out = Command::new("stt")
        .args(stt_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to process audio through stt");

    String::from_utf8_lossy(&out.stdout).to_string()
}

fn extract_stt_word_data(stt_data: &str) -> Vec<Words> {
    if stt_data.is_empty() {
        return vec![];
    }
    let transcripts: Transcripts =
        serde_json::from_str(stt_data).expect("Error deserializing json from stt data");
    return transcripts.transcripts[0].words.to_owned();
}

fn validate_models_exist(config: &Config) {
    let has_model_and_scorer =
        Path::new(&config.model_path).exists() && Path::new(&config.scorer_path).exists();

    if !has_model_and_scorer {
        panic!("Model and/or scorer files not found");
    }
}

pub fn get_wav_segment_paths(config: &Config) -> Vec<String> {
    let dir = format!("{}/wav/{}", &config.output_dir, &config.shortname);
    let d = WalkDir::new(dir)
        .into_iter()
        .filter_map(|file| file.ok())
        .map(|f| f.path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    return d;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transcripts {
    pub transcripts: Vec<Transcript>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transcript {
    confidence: f32,
    words: Vec<Words>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Words {
    pub word: String,
    pub start_time: f32,
    pub duration: f32,
}
