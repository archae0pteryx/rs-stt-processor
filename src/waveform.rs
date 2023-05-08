use std::process::Command;

use crate::{
    aws,
    files::{Args, Config},
};
use clap::Parser;
use indicatif::ProgressBar;

fn waveform_path(config: &Config) -> String {
    format!(
        "{}/json/{}.waveform.json",
        &config.output_dir, &config.shortname
    )
}

pub async fn gen_waveform(pb: &ProgressBar, config: &Config) -> anyhow::Result<()> {
    let cli_args = Args::parse();
    let contains_only = cli_args.only.contains(&String::from("waveform"));
    let only_is_empty = cli_args.only.is_empty();

    if only_is_empty || contains_only {
        pb.set_message("Running waveform processing...");
        process_waveform(&config).await?;
        return Ok(());
    }

    if only_is_empty || contains_only {
        pb.set_message("Uploading waveform json...");
        let output_file = waveform_path(config);
        aws::s3_upload(&config, &output_file).await?;
    }

    Ok(())
}

// audiowaveform -i sample.mp3 -o sample.dat -b 8
async fn process_waveform(config: &Config) -> anyhow::Result<()> {
    println!("Process waveform");
    let output_file = waveform_path(config);
    Command::new("audiowaveform")
        .args(&["-i", &config.local_file, "-o", &output_file, "-b", "8"])
        .output()
        .expect("Failed to execute waveform process command");

    Ok(())
}
