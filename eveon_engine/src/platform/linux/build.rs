use std::{fs, path::Path, process::Command};

use crate::logger as eveon_logger;
use crate::platform::build::BuildConfig;

pub fn build(cfg: BuildConfig) -> bool {
    // Setup
    let temp_path = Path::new(".temp");
    if temp_path.exists() {
        fs::remove_dir_all("./.temp").unwrap();
    }
    fs::create_dir(".temp/").unwrap();
    fs::create_dir(".temp/scripts").unwrap();
    fs::create_dir(".temp/assests").unwrap();

    // Step 1 build scripts
    for script in glob::glob(&format!("{}/**/*.rs", cfg.source_dir))
        .unwrap_or_else(|_| panic!("Failed to read glob pattern"))
    {
        match script {
            Ok(path) => {
                eveon_logger::info!(
                    "[EVEON_BUILDER]: Building script {}",
                    path.to_str().unwrap()
                );
                let out =
                    String::from(path.file_name().unwrap().to_str().unwrap()).replace("rs", "so");
                Command::new("rustc")
                    .args([
                        path.to_str().unwrap(),
                        "--crate-type",
                        "dylib",
                        "-o",
                        &format!(".temp/scripts/{}", out),
                    ])
                    .args(["-C", "prefer-dynamic", "-C", "rpath"])
                    .status()
                    .unwrap_or_else(|_| panic!("Failed to build {}", path.to_str().unwrap()));
                eveon_logger::info!(
                    "[EVEON_BUILDER]: Finished building script {}",
                    path.to_str().unwrap()
                );
            }
            Err(e) => println!("{:?}", e),
        }
    }

    true
}
