use crate::platform::{linux, windows, darwin};

pub struct BuildConfig {
    pub resource_dir: String,
    pub source_dir: String,
    pub bin_name: String
}


pub fn build(cfg: BuildConfig) -> bool {
    match crate::OS {
        "darwin" => return darwin::build(cfg),
        "linux" => return linux::build(cfg),
        "windows" => return windows::build(cfg),
        platform => panic!("Unsupported platform: {}", platform),
    }
}