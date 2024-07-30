mod repo;
mod orchestration;
mod checks;
mod config;
use tokio::signal::unix::{signal, SignalKind};
use std::fs;
use std::path::Path;
use clap::{Parser, Subcommand};
use colored::Colorize;
use crate::config::{Command, Repository};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand, Debug)]
enum Commands {
    Setup {},
    Run {},
    Update {},
    Teardown {},
}

#[tokio::main]
async fn main() {
   let args = Args::parse();
    // This is our global config. It is written to disk when we setup and used as a reference until teardown
    let config = config::Config::builder()
        .set_github_organisation_prefix("git@github.com:k8sgpt-ai/".to_string())
        .set_folder_root("k8sgpt-dev".to_string())
        .set_repositories(vec![
            Repository{
                name: "k8sgpt".to_string(),
                command: Command {
                    start: "go run main.go serve".to_string(),
                    env: "".to_string()
                },
            },
            Repository{
                name: "schemas".to_string(),
                command: Command {
                    start: "".to_string(),
                    env: "".to_string()
                },
            },
            Repository{
                name: "k8sgpt-operator".to_string(),
                command: Command {
                    start: "make install && go run main.go".to_string(),
                    env: "LOCAL_MODE=true".to_string()
                },
            },
        ]).build();
    let checks = checks::Checks::builder().build();
    let repo_manager = repo::RepoManager::builder()
        .build();
    let orchestrator = orchestration::Orchestration::builder()
        .set_config(config.clone()).build();

    // Signal handling
    // ctrl-c
    tokio::spawn(async {
        let mut term = signal(SignalKind::interrupt()).unwrap();
        term.recv().await;
        println!("{}","Received interrupt signal".red());

    });

    match args.command.expect("requires a command") {
        Commands::Setup { .. } => {
            checks.run_preflight();
            fs::create_dir(config.folder_root.clone()).unwrap_or_else(|why| {
                println!("! {:?}", why);
            });
            // Fetch the remote repositories
            repo_manager.clone_repo(config.clone()).await.expect("error cloning repositories");
            println!("Writing config to disk");
            config.write_config().unwrap();
        }
        Commands::Run { .. } => {
            // check if the folder_root exists
            let path = Path::new(config.folder_root.as_str());
            if !path.exists() {
                println!("{}","Please run the setup command first".red());
                return;
            }
            orchestrator.run().await.expect("error running orchestrator")
        }
        Commands::Update { .. } => {

        }
        Commands::Teardown { .. } => {
            //orchestrator.remove().expect("error removing resources");
            fs::remove_dir_all(Path::new(config.folder_root.as_str())).expect("error deleting workspace")
        }
    }
}
