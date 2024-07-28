use clap::{Parser, Subcommand};
use kdev::{run, setup, teardown};

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
    Teardown {},
}

fn main() {
   let args = Args::parse();

    match args.command.expect("requires a command") {
        Commands::Setup { .. } => {
            setup();
        }
        Commands::Run { .. } => {
            run();
        }
        Commands::Teardown { .. } => {
            teardown();
        }
    }
}
