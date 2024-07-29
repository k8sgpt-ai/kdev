use colored::Colorize;
use crate::repo;

#[derive(Debug,Default)]
pub struct OrchestrationBuilder;

impl OrchestrationBuilder {
    pub fn new() -> OrchestrationBuilder {
        OrchestrationBuilder {
        }
    }
    pub fn build(self) -> Orchestration {
        Orchestration {
        }
    }
}

pub struct Orchestration {
}

impl Orchestration {
    pub fn builder() -> OrchestrationBuilder {
        OrchestrationBuilder::default()
    }

    fn install_operator(&self) -> Result<(), Box<dyn std::error::Error>> {
        // cd k8sgpt-dev/k8sgpt-operator and run the Makefile command make install
        let output = std::process::Command::new("make")
            .arg("install")
            .current_dir(format!("{}/k8sgpt-operator",repo::K8SGPT_DEV_FOLDER_NAME))
            .output().unwrap();
        if output.status.code().unwrap() != 0 {
            return Err(string_error::new_err(std::str::from_utf8(&*output.stderr).unwrap()));
        }
        Ok(())
    }

    fn run_operator(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Run the operator in a child process

        let output = std::process::Command::new("make")
            .arg("run")
            .current_dir("k8sgpt-dev/k8sgpt-operator")
            .output().unwrap();

        Ok(())
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        // Install the operator and the CRDs into the Kubernetes cluster
        self.install_operator()?;
        println!("{}","Installed Operator CRD".blue());


        Ok(())
    }
}