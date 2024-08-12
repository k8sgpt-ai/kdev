use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Command {
    pub start: String,
    pub env: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckOutInfo {
    pub branch_name: String,
    pub commit: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub command: Command,
    pub checkout_info: Option<CheckOutInfo>,
}

#[derive(Default)]
pub struct ConfigBuilder {
    pub repositories: Vec<Repository>,
    pub folder_root: String,
    pub github_organisation_prefix: String,
}
impl ConfigBuilder {
    pub fn new(repository: Vec<Repository>) -> ConfigBuilder {
        ConfigBuilder {
            repositories: repository,
            folder_root: "k8sgpt-dev".to_string(),
            github_organisation_prefix: "git@github.com:k8sgpt-ai/".to_string(),
        }
    }
    pub fn set_repositories(mut self, repositories: Vec<Repository>) -> ConfigBuilder {
        self.repositories = repositories;
        self
    }
    pub fn set_folder_root(mut self, folder_root: String) -> ConfigBuilder {
        self.folder_root = folder_root;
        self
    }
    pub fn set_github_organisation_prefix(
        mut self,
        github_organisation_prefix: String,
    ) -> ConfigBuilder {
        self.github_organisation_prefix = github_organisation_prefix;
        self
    }
    pub fn build(self) -> Config {
        Config {
            repositories: self.repositories,
            folder_root: self.folder_root,
            github_organisation_prefix: self.github_organisation_prefix,
        }
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub folder_root: String,
    pub repositories: Vec<Repository>,
    pub github_organisation_prefix: String,
}

impl Config {
    pub(crate) fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn write_config(self) -> std::io::Result<()> {
        // encode the config with serde_json
        let encoded = serde_json::to_string(&self).unwrap();
        // write the config to disk into the folder_root
        let path = format!("{}/config.json", self.folder_root);
        std::fs::write(path, encoded)
    }
    pub fn read_config(self) -> Result<Config, Box<dyn Error>> {
        let path = format!("{}/config.json", self.folder_root);
        let contents = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
    pub fn exists(self) -> bool {
        let path = format!("{}/config.json", self.folder_root);
        std::path::Path::new(&path).exists()
    }
}
