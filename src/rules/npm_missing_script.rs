use super::{Command, Rule};
use regex::Regex;
use std::process::Command as ProcessCommand;

pub struct NpmMissingScript;

impl Rule for NpmMissingScript {
    fn name() -> &'static str {
        "npm_missing_script"
    }

    fn matches(cmd: &Command) -> bool {
        let has_run_prefix = cmd.text.split_whitespace()
            .any(|part| part.starts_with("ru"));
        let has_missing_script = cmd.output
            .as_ref()
            .map(|output| output.contains("npm ERR! missing script: "))
            .unwrap_or(false);
        
        has_run_prefix && has_missing_script
    }

    fn fix(cmd: &Command) -> String {
        let output = cmd.output.as_ref().expect("Expected output");
        let re = Regex::new(r".*missing script: (.*)\n").unwrap();
        let misspelled_script = re.captures(output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .expect("Could not find misspelled script");
        
        let scripts = get_scripts();
        replace_command(cmd, misspelled_script, &scripts)
    }
}

fn get_scripts() -> Vec<String> {
    let output = ProcessCommand::new("npm")
        .arg("run")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();
    
    output.lines()
        .skip_while(|line| !line.starts_with("Lifecycle scripts"))
        .skip(1)
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("  ") {
                trimmed.split_whitespace().next().map(String::from)
            } else {
                None
            }
        })
        .collect()
}

fn replace_command(cmd: &Command, misspelled: &str, scripts: &[String]) -> String {
    let similar_scripts: Vec<&String> = scripts.iter()
        .filter(|s| s.starts_with(&misspelled[..1]))
        .collect();
    
    if similar_scripts.len() == 1 {
        cmd.text.replace(misspelled, similar_scripts[0])
    } else {
        cmd.text.clone()
    }
}