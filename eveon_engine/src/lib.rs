pub mod core;
pub mod logger;
pub mod platform;
pub mod project;
pub mod scripting;

pub use std::env::consts::OS;
use std::path::Path;

use project::Project;

pub fn get_project(path: String) -> Project {
    Project::open(Path::new(&path))
}
