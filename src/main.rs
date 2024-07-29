mod repo;
mod orchestration;
mod checks;

use std::fs;
use std::path::Path;
use clap::{Parser, Subcommand};

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

fn main() {
   let args = Args::parse();
    let checks = checks::Checks::builder().build();
    checks.run_preflight();
    let repo_manager = repo::RepoManager::builder().build();
    let orchestrator = orchestration::Orchestration::builder().build();
    match args.command.expect("requires a command") {
        Commands::Setup { .. } => {
            fs::create_dir(repo::K8SGPT_DEV_FOLDER_NAME).unwrap_or_else(|why| {
                println!("! {:?}", why);
            });
            // Fetch the remote repositories
            repo_manager.clone_repo().expect("error cloning repositories")
        }
        Commands::Run { .. } => {
            orchestrator.run().expect("error running orchestrator")
        }
        Commands::Update { .. } => {

        }
        Commands::Teardown { .. } => {
            fs::remove_dir_all(Path::new(repo::K8SGPT_DEV_FOLDER_NAME)).expect("error deleting workspace")
        }
    }
}
