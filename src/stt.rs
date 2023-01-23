use crate::constants::{JSON_PATH, MODEL_FILE, SCORER_FILE, SPLIT_AUDIO_DIR};
use crate::files;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::Barrier;
use std::sync::{Arc, Mutex};
use std::{fs::File, path::Path};
use walkdir::WalkDir;

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

pub fn process_wav_segments(src_path: &str) -> anyhow::Result<()> {
    let episode_shortname = files::get_shortname(src_path);

    validate_models_exist();
    let wav_segment_paths = get_wav_segment_paths(&episode_shortname);

    let barrier = Barrier::new(wav_segment_paths.len());
    let json_results = Arc::new(Mutex::new(vec![]));
    wav_segment_paths.par_iter().for_each(|wav_path| {
        let stt_data = process_stt(wav_path);
        let word_data = extract_stt_word_data(&stt_data);
        json_results.lock().unwrap().extend(word_data);
        barrier.wait();
    });

    let json_results = json_results.clone();
    let mut json_results = json_results.lock().unwrap();
    json_results.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());

    let j = serde_json::to_string(&*json_results)
        .expect("Failed to convert mutex word vec to json string");

    let out_json_path = format!("{}/{}.json", JSON_PATH, episode_shortname);

    let mut json_file = File::create(&out_json_path)
        .expect(format!("Error creating json file: {}", out_json_path).as_str());

    json_file
        .write_all(j.as_bytes())
        .expect("Failed to write json file");

    // calc total seconds between then and now

    Ok(())
}

fn process_stt(wav_path: &str) -> String {
    let stt_args = vec![
        "--model",
        MODEL_FILE,
        "--scorer",
        SCORER_FILE,
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

fn validate_models_exist() {
    let has_model_and_scorer = Path::new(MODEL_FILE).exists() && Path::new(SCORER_FILE).exists();

    if !has_model_and_scorer {
        panic!("Model and/or scorer files not found");
    }
}

pub fn get_wav_segment_paths(shortname: &str) -> Vec<String> {
    let dir = format!("{}/{}", SPLIT_AUDIO_DIR, shortname);
    let d = WalkDir::new(dir)
        .into_iter()
        .filter_map(|file| file.ok())
        .map(|f| f.path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    return d;
}
