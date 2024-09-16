use std::collections::HashMap;
use colored::Colorize;
use std::env;

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

        let checklist: HashMap<&str,Vec<&str>> = HashMap::from([
            ("Docker", vec!["docker", "info"]),
            ("Golang", vec!["go", "version"]),
            ("Kubectl", vec!["kubectl", "version"]),
            ("Make", vec!["make", "--version"]),
            ]);

        // Loop through the hashmap and tokenise the values into an array of args

        for (key, value) in checklist.iter() {
            // convert value into an array
            let output = std::process::Command::new(value[0])
                .arg(value[1])
                .output();
            match output {
                Ok(x) => {
                    if x.status.code().unwrap() != 0 {
                        // print stderr
                        println!("{} {}", format!("{} is not installed:", key).red(),String::from_utf8_lossy(&x.stderr));
                        continue
                    } else {
                        println!("{}", format!("{} is installed", key).green());
                    }
                },
                Err(e) => {
                    eprintln!("{}",format!("{} is not installed", key).red());
                    continue
                }
            }
        }
    }
}
