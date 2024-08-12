use crate::config::Config;
use colored::Colorize;
use notify::Config as nconfig;
use notify::{EventKind, RecommendedWatcher, Watcher};
use tokio::io::{AsyncBufReadExt, BufReader};

#[derive(Debug, Default)]
pub struct OrchestrationBuilder {
    config: Config,
}

impl OrchestrationBuilder {
    pub fn new(config: Config) -> OrchestrationBuilder {
        OrchestrationBuilder { config }
    }
    pub fn set_config(mut self, config: Config) -> OrchestrationBuilder {
        self.config = config;
        self
    }
    pub fn build(self) -> Orchestration {
        Orchestration {
            config: self.config,
            pids: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Orchestration {
    config: Config,
    pids: Vec<u32>,
}

impl Orchestration {
    pub fn builder() -> OrchestrationBuilder {
        OrchestrationBuilder::default()
    }

    pub async fn stop_services(self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Stopping services");
        for pid in self.pids {
            println!("{} {}", "Stopping".red(), pid);
            let _ = tokio::process::Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .await?;
        }
        Ok(())
    }
    pub async fn start_services(mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting services");
        let (tx, mut rx) = tokio::sync::mpsc::channel(self.config.repositories.len());
        for repo in self.config.repositories {
            // start each service in a tokio spawn
            tokio::spawn({
                let repo = repo.clone();
                let folder_root = self.config.folder_root.clone();
                let tx: tokio::sync::mpsc::Sender<u32> = tx.clone();
                async move {
                    if !repo.command.start.is_empty() {
                        // stream the output from the program
                        let mut child = tokio::process::Command::new("sh")
                            .arg("-c")
                            .env("LOCAL_MODE", "true")
                            .arg(repo.command.start)
                            .current_dir(format!("{}/{}", folder_root, repo.name))
                            .spawn()
                            .expect("failed to start process");
                        let child_id = child.id();
                        println!(
                            "{} {} with pid {}",
                            "Starting".yellow(),
                            repo.name,
                            child_id.unwrap()
                        );
                        tx.send(child.id().unwrap()).await.unwrap();
                        let stdout = child.stdout.take().unwrap();
                        // store the childID
                        //self.pids.insert(repo.name.clone(), child_id.unwrap());
                        let mut reader = BufReader::new(stdout);
                        let mut line = String::new();
                        loop {
                            match reader.read_line(&mut line).await {
                                Ok(0) => break,
                                Ok(_) => {
                                    println!("{}", line);
                                    line.clear();
                                }
                                Err(e) => {
                                    println!("error reading line: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                }
            });
        }
        while let Some(message) = rx.recv().await {
            self.pids.push(message);
        }

        Ok(())
    }
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut tasks = vec![];
        for i in 0..self.config.repositories.len() {
            let repo = self.config.repositories[i].clone();
            // path name
            let path = format!("{}/{}", self.config.folder_root, repo.name.clone());
            println!(
                "{}",
                format!("{} {}", "Starting file watcher for ".blue(), path)
            );
            tasks.push(tokio::spawn({
                async move {
                    let (tx, rx) = std::sync::mpsc::channel();
                    let mut watcher = RecommendedWatcher::new(tx, nconfig::default()).unwrap();
                    watcher
                        .watch(path.as_ref(), notify::RecursiveMode::Recursive)
                        .unwrap();
                    loop {
                        match rx.recv() {
                            Ok(event) => {
                                if let Ok(e) = event {
                                    match e.kind {
                                        EventKind::Any => {}
                                        EventKind::Access(_) => {}
                                        EventKind::Create(_) => {}
                                        EventKind::Modify(_) => {
                                            // restart the components
                                            println!(
                                                "Detected changes in {}, reloading",
                                                e.paths[0].display()
                                            );
                                        }
                                        EventKind::Remove(_) => {}
                                        EventKind::Other => {}
                                    }
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

        // Start
        self.start_services().await?;

        for task in tasks {
            task.await.unwrap();
        }

        Ok(())
    }
}
