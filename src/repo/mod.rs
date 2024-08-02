use std::error::Error;
use std::env;
use std::path::Path;
use colored::Colorize;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use tokio::task::JoinHandle;
use crate::config::Config;

#[derive(Default)]
pub struct RepoManagerBuilder;

impl RepoManagerBuilder {

    pub fn new() -> RepoManagerBuilder {
        RepoManagerBuilder {

        }
    }
    pub fn build(self) -> RepoManager {
        RepoManager{

        }
    }
}

pub struct RepoManager;
impl RepoManager {

    pub fn builder() -> RepoManagerBuilder {
        RepoManagerBuilder
    }
    pub async fn clone_repo(self, config: Config) -> Result<(), Box<dyn Error >> {

        let mut tasks: Vec<JoinHandle<()>> = vec![];

        for i in 0..config.repositories.len() {
            let repo = config.repositories[i].clone();
            let repo_url = format!("{}{}.git", config.github_organisation_prefix.clone(), repo.name.clone());
            let repo_folder = format!("{}/{}", config.folder_root.clone(), repo.name.clone());
            // async cloning
            tasks.push(tokio::spawn(
                {
                    let repo_url = repo_url.clone();
                    let repo_folder = repo_folder.clone();
                async move
                {
                    let mut builder = RepoBuilder::new();
                    let mut fetch_options = FetchOptions::new();
                    let mut callbacks = RemoteCallbacks::new();
                    callbacks. credentials(|_url, _username_from_url, _allowed_types| {
                        Cred::ssh_key(
                            "git",
                            Some(std::path::Path::new(&format!("{}/.ssh/id_rsa.pub", env::var("HOME").unwrap()))),
                            std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                            None,
                        )
                    });
                    fetch_options.remote_callbacks(callbacks);
                    builder.fetch_options(fetch_options);
                 let checked_out = builder.clone(repo_url.as_str(), Path::new(repo_folder.as_str())).unwrap();
                    println!("{}",format!("Cloning {} complete", repo.name.clone()).blue());
                    println!("Check out complete, branch is {} at commit {} ", checked_out.head().unwrap().name().unwrap(), checked_out.head().unwrap().peel_to_commit().unwrap().id());
            }}));
        }


        for task in tasks {
            task.await.unwrap();
        }

        Ok(())
    }
}