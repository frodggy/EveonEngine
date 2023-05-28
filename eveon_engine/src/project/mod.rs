pub mod config;
use config::ProjectConfig;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::{
    logger,
    platform::{self, BuildConfig},
};

#[derive(Debug, Clone)]
pub struct Project {
    config: ProjectConfig,
    project_path: String,
}

impl Project {
    pub fn new(path: &Path) -> Self {
        Self {
            project_path: path.to_str().unwrap().to_string(),
            config: ProjectConfig::default(),
        }
    }

    pub fn open(path: &Path) -> Self {
        let old_path = env::current_dir().unwrap();
        env::set_current_dir(path).unwrap();

        let data = fs::read_to_string("./project.ev").unwrap();
        let config: ProjectConfig = toml::from_str(data.as_str()).unwrap();

        env::set_current_dir(old_path).unwrap();

        logger::info!("Platform: {}", crate::OS);

        Self {
            config,
            project_path: String::from(path.to_str().unwrap()),
        }
    }

    pub fn get_project_dir(&self) -> String {
        self.project_path.clone()
    }

    pub fn get_resources_dir(&self) -> PathBuf {
        let cfg = self.config.clone();
        let path = Path::new(&self.get_project_dir())
            .join(cfg.project.config.resources_dir.replace("./", ""));
        path
    }

    pub fn get_name(&self) -> String {
        self.config.project.name.clone()
    }

    pub fn get_source_dir(&self) -> PathBuf {
        let cfg = self.config.clone();
        let path = Path::new(&self.get_project_dir())
            .join(cfg.project.config.source_dir.replace("./", ""));
        path
    }

    pub fn get_plugin_dir(&self) -> PathBuf {
        let cfg = self.config.clone();
        let path = Path::new(&self.get_project_dir())
            .join(cfg.project.config.plugin_dir.replace("./", ""));
        path
    }

    pub fn build(&self) {
        let cfg = self.config.clone();

        let build_opts = BuildConfig {
            resource_dir: String::from(self.get_resources_dir().to_str().unwrap()),
            source_dir: String::from(self.get_source_dir().to_str().unwrap()),
            bin_name: cfg.project.build.binary_name,
        };

        platform::build(build_opts);
    }

    pub fn print_cfg(&self) {
        println!("{:#?}", self.config)
    }
}
