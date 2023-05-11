use std::{sync::Arc, path::Path};
use anyhow::{Context, Result};
use serde::Serialize;
use serde_json::Value;
use crate::youtube_dl;

pub static WORKDIR: &str = "files/audio";

#[derive(Serialize, Clone, Debug)]
pub struct AppPaths {
    pub root_path: String,
    pub tmp_path: String,
    pub data_path: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct AppState {
    pub url: String,
    pub paths: AppPaths,
}

impl AppState {
    pub fn new(url: &str) -> Arc<AppState> {
        let state = AppState {
            url: url.to_owned(),
            paths: AppPaths {
                root_path: WORKDIR.to_owned(),
                tmp_path: format!("{}/tmp", WORKDIR).to_owned(),
                data_path: format!("{}/data", WORKDIR).to_owned(),
            }
        };
        setup_dirs(state.clone()).unwrap();
        Arc::new(state)
    }
}


fn create_dir_vec(state: AppState) -> Vec<String> {
    let value: Value = serde_json::to_value(&state.paths).unwrap();
    let map = value.as_object().unwrap();

    let values: Vec<&Value> = map.values().collect();
    let v = values.iter().map(|v| v.to_string()).collect();
    dbg!(&v);
    v
}

fn setup_dirs(state: AppState) -> Result<()> {
    let dirs = create_dir_vec(state);
    for dir in dirs {
        std::fs::create_dir_all(&dir).context(format!("Dir exists: {}", dir));
    }
    Ok(())
}

