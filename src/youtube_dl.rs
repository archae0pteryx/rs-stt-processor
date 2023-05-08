use std::process::{Command, Stdio};

use log::debug;

use crate::{WORKDIR, Target};

static YTDL_CONFIG: &str = "ytdl.conf";

pub async fn youtube_dl(url: &str) -> String {
    let name = url.split("=").last().unwrap();
    let out_path = format!("{}/raw/{}.wav", WORKDIR, name);

    debug!("downloading {} to: {}", &name, &out_path);

    let out = Command::new("bash")
        .args(vec![
            "-c",
            format!(
                "yt-dlp --config-location {} --output {} {}",
                YTDL_CONFIG, &out_path, url
            )
            .as_str(),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Shell command failed");

    out_path.to_string()
}
