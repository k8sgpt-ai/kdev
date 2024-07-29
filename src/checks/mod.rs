use std::env;
use colored::Colorize;

pub struct ChecksBuilder;

impl ChecksBuilder {
    pub fn new() -> Self {
        ChecksBuilder
    }

    pub fn build(self) -> Checks {
        Checks
    }
}

pub struct Checks;

impl Checks {
    pub fn builder() -> ChecksBuilder {
        ChecksBuilder::new()
    }

    pub fn run_preflight(&self) {
        // check if docker is running
        let output = std::process::Command::new("docker")
            .arg("info")
            .output().unwrap();
        if output.status.code().unwrap() != 0 {
            println!("{}","Docker is not running".red());
        } else {
            println!("{}","Docker is running".green());
        }
        // Check if golang is installed
        let output = std::process::Command::new("go")
            .arg("version")
            .output().unwrap();
        if output.status.code().unwrap() != 0 {
            println!("{}","Golang is not installed".red());
        } else {
            println!("{}","Golang is installed".green());
        }
        // Check if there is an ssh key installed
        let output = std::process::Command::new("ls")
            .arg("-al")
            .arg(format!("{}/.ssh/id_rsa", env::var("HOME").unwrap()))
            .output().unwrap();
        if output.status.code().unwrap() != 0 {
            println!("{}","SSH key is not installed".red());
        } else {
            println!("{}","SSH key is installed".green());
        }
        // Check if kubectl is installed
        let output = std::process::Command::new("kubectl")
            .arg("version")
            .output().unwrap();
        if output.status.code().unwrap() != 0 {
            println!("{}","Kubectl is not installed".red());
        } else {
            println!("{}","Kubectl is installed".green());
        }
        // Check if Make is installed
        let output = std::process::Command::new("make")
            .arg("--version")
            .output().unwrap();
        if output.status.code().unwrap() != 0 {
            println!("{}","Make is not installed".red());
        } else {
            println!("{}","Make is installed".green());
        }
    }
}