use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ProjectConfig {
    pub project: ProjectConfigWrapper
}
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ProjectConfigWrapper {
    pub name: String,
    pub icon: String,
    pub main_scene: String,
    pub config: ProjectConfigDir,
    pub build: ProjectConfigBuild,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ProjectConfigDir {
    pub resources_dir: String,
    pub source_dir: String,
    pub plugin_dir: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ProjectConfigBuild {
    pub targets: Vec<ProjectTargets>,
    pub binary_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ProjectTargets {
    Linux,
    Windows,
    Darwin,
}
