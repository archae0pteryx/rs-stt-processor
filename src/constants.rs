#![allow(unused)]
pub static FILES_DIR: &str = "./files";
pub static MODEL_FILE: &str = "./models/coqui-model.tflite";
pub static SCORER_FILE: &str = "./models/coqui-huge-vocabulary.scorer";
pub static LOGS_PATH : &str = "./logs";
pub static JSON_PATH : &str = "./output/json";
pub static MODEL_URL: &str = "https://coqui.gateway.scarf.sh/english/coqui/v1.0.0-huge-vocab/model.tflite";
pub static SCORER_URL: &str = "https://coqui.gateway.scarf.sh/english/coqui/v1.0.0-huge-vocab/huge-vocabulary.scorer";
pub static S3_BUCKET: &str = "ds-stt-bucket";
pub static SPLIT_AUDIO_DIR: &str = "./output/wav";
