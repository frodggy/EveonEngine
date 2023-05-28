use crate::platform::{darwin, linux, windows};

pub struct BuildConfig {
    pub resource_dir: String,
    pub source_dir: String,
    pub bin_name: String,
}

pub fn build(cfg: BuildConfig) -> bool {
    match crate::OS {
        "darwin" => darwin::build(cfg),
        "linux" => linux::build(cfg),
        "windows" => windows::build(cfg),
        platform => panic!("Unsupported platform: {}", platform),
    }
}
