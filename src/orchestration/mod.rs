use colored::Colorize;
use notify::{Config, EventKind, RecommendedWatcher, Watcher};
use tokio::task::JoinHandle;
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

    fn command_operator(&self, command: &str) -> Result<(), Box<dyn std::error::Error>> {
        // cd k8sgpt-dev/k8sgpt-operator and run the Makefile command make install
        let output = std::process::Command::new("make")
            .arg(command)
            .current_dir(format!("{}/k8sgpt-operator",repo::K8SGPT_DEV_FOLDER_NAME))
            .output().unwrap();
        if output.status.code().unwrap() != 0 {
            return Err(string_error::new_err(std::str::from_utf8(&*output.stderr).unwrap()));
        }
        Ok(())
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        // Install the operator and the CRDs into the Kubernetes cluster
        self.command_operator("install")?;
        println!("{}","Installed Operator CRD".blue());
        let mut tasks = vec![];
        for i in 0..repo::K8SGPT_REMOTE_REPO_NAMES.len() {
            let repo = repo::K8SGPT_REMOTE_REPO_NAMES[i];
            // path name
            let path = format!("{}/{}", repo::K8SGPT_DEV_FOLDER_NAME, repo.clone());
            println!("{}",format!("{} {}", "Starting file watcher for ".blue(), path));
            tasks.push(tokio::spawn({
                async move {

                    let (tx, rx) = std::sync::mpsc::channel();
                    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
                    watcher.watch(path.as_ref(), notify::RecursiveMode::Recursive).unwrap();
                    loop {
                        match rx.recv() {
                            Ok(event) => {
                                match event {
                                    Ok(e) => {
                                        match e.kind {
                                            EventKind::Any => {}
                                            EventKind::Access(_) => {}
                                            EventKind::Create(_) => {}
                                            EventKind::Modify(_) => {
                                                // restart the components
                                            }
                                            EventKind::Remove(_) => {}
                                            EventKind::Other => {}
                                        }
                                    }
                                    Err(_) => {}
                                }
                            }
                            Err(e) => {
                                 println!("watch error: {:?}", e);
                            }
                        }
                    }
                }
            }));
        }
        for task in tasks {
            task.await.unwrap();
        }

        Ok(())
    }

    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        // cd k8sgpt-dev/k8sgpt-operator and run the Makefile command make uninstall
        self.command_operator("uninstall")?;
        println!("{}","Uninstalled Operator CRD".blue());
        Ok(())
    }
}