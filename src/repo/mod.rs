use std::error::Error;
use std::{env, os};
use std::path::Path;
use colored::Colorize;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks};
use log::{ error};
use string_error;
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
pub const K8SGPT_DEV_FOLDER_NAME: &str = "k8sgpt-dev";
const K8SGPT_REPO_PREFIX: &str = "git@github.com:k8sgpt-ai/";
const K8SGPT_REMOTE_REPO_NAMES: [&str; 3] = [
    "schemas",
    "k8sgpt",
    "k8sgpt-operator"];

pub struct RepoManager;
impl RepoManager {

    pub fn builder() -> RepoManagerBuilder {
        RepoManagerBuilder::default()
    }
    pub fn clone_repo(self) -> Result<(), Box<dyn Error >> {
        let mut builder = RepoBuilder::new();
        let mut callbacks = RemoteCallbacks::new();
        let mut fetch_options = FetchOptions::new();


        callbacks. credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                "git",
                Some(std::path::Path::new(&format!("{}/.ssh/id_rsa.pub", env::var("HOME").unwrap()))),
                std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                None,
            )
        });

        fetch_options.remote_callbacks(callbacks);

        builder.fetch_options(fetch_options);

        for repo in K8SGPT_REMOTE_REPO_NAMES {
            let repo_url = format!("{}{}.git", K8SGPT_REPO_PREFIX, repo);
            let repo_folder = format!("{}/{}", K8SGPT_DEV_FOLDER_NAME, repo);
            builder.clone(repo_url.as_str(), Path::new(repo_folder.as_str())).expect("Could not clone repo");
            println!("{}",format!("Cloning {} complete", repo).blue());
        }
        Ok(())
    }
}