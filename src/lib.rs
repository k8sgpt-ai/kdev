use std::fs;
use crate::pkg::RepoManager;

mod pkg;
const K8SGPT_DEV_FOLDER_NAME: &str = "k8sgpt-dev";
const K8SGPT_REPO_PREFIX: &str = "git@github.com:k8sgpt-ai/";
const K8SGPT_REMOTE_REPO_NAMES: [&str; 3] = [
    "schemas",
    "k8sgpt",
    "k8sgpt-operator"];
pub fn setup(repo_manager: RepoManager) {
    // Check if the dev folder exists
    fs::create_dir(K8SGPT_DEV_FOLDER_NAME).unwrap_or_else(|why| {
       println!("! {:?}", why);
    });
    // Fetch the remote repositories
    repo_manager.clone_repo().expect("error cloning repositories")

}
pub fn update() {
    println!("This will update all repositories");
    println!("Are you sure you want to continue? (y/n)");
    let mut response = String::new();
    std::io::stdin().read_line(&mut response).expect("Failed to read line");
    if response.trim() == "y" {


    }
}
pub fn run() {
    
}

pub fn teardown() {

    fs::remove_dir_all(K8SGPT_DEV_FOLDER_NAME).unwrap_or_else(|why| {
       println!("! {:?}", why);
    });

}