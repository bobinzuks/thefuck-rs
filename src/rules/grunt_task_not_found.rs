use super::{Command, Rule};
use regex::Regex;
use std::process::Command as ProcessCommand;
use std::str;
use strsim::levenshtein;

pub struct GruntTaskNotFound;

impl Rule for GruntTaskNotFound {
    fn name() -> &'static str {
        "grunt_task_not_found"
    }

    fn matches(cmd: &Command) -> bool {
        let re = Regex::new(r#"Warning: Task "(.*)" not found."#).unwrap();
        re.is_match(&cmd.output)
    }

    fn fix(cmd: &Command) -> Option<String> {
        let re = Regex::new(r#"Warning: Task "(.*)" not found."#).unwrap();
        let caps = re.captures(&cmd.output)?;
        let misspelled_task = caps.get(1)?.as_str().split(':').next()?.to_string();
        
        let tasks = get_all_tasks()?;
        let fixed = get_closest(&misspelled_task, &tasks)?;
        
        Some(cmd.text.replace(&format!(" {}", misspelled_task), &format!(" {}", fixed)))
    }
}

fn get_all_tasks() -> Option<Vec<String>> {
    let output = ProcessCommand::new("grunt")
        .arg("--help")
        .output()
        .ok()?;
    
    let stdout = str::from_utf8(&output.stdout).ok()?;
    let mut tasks = Vec::new();
    let mut should_yield = false;
    
    for line in stdout.lines() {
        let line = line.trim();
        
        if line.contains("Available tasks") {
            should_yield = true;
            continue;
        }
        
        if should_yield && line.is_empty() {
            break;
        }
        
        if should_yield && line.contains("  ") {
            if let Some(task) = line.split_whitespace().next() {
                tasks.push(task.to_string());
            }
        }
    }
    
    if tasks.is_empty() { None } else { Some(tasks) }
}

fn get_closest(misspelled: &str, tasks: &[String]) -> Option<String> {
    tasks.iter()
        .min_by_key(|task| levenshtein(misspelled, task))
        .cloned()
}