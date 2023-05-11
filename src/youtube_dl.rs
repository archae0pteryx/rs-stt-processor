use std::{process::{Command, Stdio}, sync::Arc};

use log::debug;

use crate::state::AppState;

static YTDL_CONFIG: &str = "ytdl.conf";

pub fn youtube_dl(state: Arc<AppState>) {
    let output_name = format!("{}/%(title)s.%(ext)s", state.paths.tmp_path);
    let args = vec![
        "yt-dlp",
        "-x",
        "--audio-format",
        "wav",
        "--restrict-filenames",
        "--sub-lang",
        "en",
        "--sub-format",
        "json3",
        "--output",
        &output_name,
        "--write-auto-sub",
        &state.url,
    ];
    let out = Command::new("bash")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Shell command failed");

}
