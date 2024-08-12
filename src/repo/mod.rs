use crate::config::{CheckOutInfo, Config};
use colored::Colorize;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks};
use std::env;
use std::error::Error;
use std::path::Path;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle;

#[derive(Default)]
pub struct RepoManagerBuilder;

impl RepoManagerBuilder {
    pub fn new() -> RepoManagerBuilder {
        RepoManagerBuilder {}
    }
    pub fn build(self) -> RepoManager {
        RepoManager {}
    }
}

pub struct RepoManager;
impl RepoManager {
    pub fn builder() -> RepoManagerBuilder {
        RepoManagerBuilder
    }
    pub async fn clone_repo(self, mut config: Config) -> Result<(), Box<dyn Error>> {

        // TODO: Can you believe git2 isn't Send safe, no multi-threading for us...
        for i in 0..config.repositories.len() {
            let mut repo = config.repositories[i].clone();
            let repo_url = format!(
                "{}{}.git",
                config.github_organisation_prefix.clone(),
                repo.name.clone()
            );
            let repo_folder = format!("{}/{}", config.folder_root.clone(), repo.name.clone());
            // async cloning
            let mut builder = RepoBuilder::new();
            let mut fetch_options = FetchOptions::new();
            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|_url, _username_from_url, _allowed_types| {
                Cred::ssh_key(
                    "git",
                    Some(std::path::Path::new(&format!(
                        "{}/.ssh/id_rsa.pub",
                        env::var("HOME").unwrap()
                    ))),
                    std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                    None,
                )
            });
            fetch_options.remote_callbacks(callbacks);
            builder.fetch_options(fetch_options);
            let checked_out = builder
                .clone(repo_url.as_str(), Path::new(repo_folder.as_str()))
                .unwrap();
            println!(
                "{}",
                format!("Cloning {} complete", repo.name.clone()).blue()
            );
            // Save the git commit hash and branch to the config
            let checkout_info = CheckOutInfo {
                branch_name: checked_out.head().unwrap().name().unwrap().to_string(),
                commit: checked_out.head().unwrap().peel_to_commit().unwrap().id().to_string(),
            };
            println!(
                "Check out complete, branch is {} at commit {} ",
                checkout_info.branch_name,
                checkout_info.commit
            );
            repo.checkout_info = Some(checkout_info);
            config.repositories[i] = repo;
        }
        // Write the config with the updated checkout info
        config.write_config()?;
        Ok(())
    }
}
