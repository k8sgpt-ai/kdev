mod pkg;

use std::fs;
use clap::{Parser, Subcommand};
use kdev::{run, setup, teardown, update};

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

    let repo_manager = pkg::RepoManagerBuilder::new().build();

    match args.command.expect("requires a command") {
        Commands::Setup { .. } => {
            fs::create_dir(pkg::K8SGPT_DEV_FOLDER_NAME).unwrap_or_else(|why| {
                println!("! {:?}", why);
            });
            // Fetch the remote repositories
            repo_manager.clone_repo().expect("error cloning repositories")
        }
        Commands::Run { .. } => {
            run();
        }
        Commands::Update { .. } => {
            update();
        }
        Commands::Teardown { .. } => {
            teardown();
        }
    }
}
