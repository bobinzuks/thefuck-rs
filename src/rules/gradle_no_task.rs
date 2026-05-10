use super::{Command, Rule};
use regex::Regex;
use std::process::{Command as ProcessCommand, Stdio};

pub struct GradleNoTask;

impl Rule for GradleNoTask {
    fn name(&self) -> &'static str {
        "gradle_no_task"
    }

    fn matches(&self, cmd: &Command) -> bool {
        let re = Regex::new(r"Task '(.*)' (is ambiguous|not found)").unwrap();
        re.is_match(&cmd.output)
    }

    fn fix(&self, cmd: &Command) -> String {
        let re = Regex::new(r"Task '(.*)' (is ambiguous|not found)").unwrap();
        let wrong_task = re.captures(&cmd.output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        let gradle = &cmd.script_parts[0];
        let all_tasks = get_all_tasks(gradle);
        
        replace_command(cmd.script_parts.clone(), wrong_task, &all_tasks)
    }
}

fn get_all_tasks(gradle: &str) -> Vec<String> {
    let output = ProcessCommand::new(gradle)
        .arg("tasks")
        .stdout(Stdio::piped())
        .output()
        .ok();

    let mut tasks = Vec::new();
    
    if let Some(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut should_yield = false;
        
        for line in stdout.lines() {
            let line = line.trim();
            
            if line.starts_with("----") {
                should_yield = true;
                continue;
            }
            
            if line.is_empty() {
                should_yield = false;
                continue;
            }
            
            if should_yield && !line.contains("All tasks runnable from root project") {
                if let Some(task) = line.split_whitespace().next() {
                    tasks.push(task.to_string());
                }
            }
        }
    }
    
    tasks
}

fn replace_command(script_parts: Vec<String>, wrong_task: &str, all_tasks: &[String]) -> String {
    // Simple replacement: find best match or return original command
    if let Some(best_match) = find_best_match(wrong_task, all_tasks) {
        let task_index = script_parts.iter().position(|p| p == wrong_task)
            .unwrap_or(0);
        let mut new_parts = script_parts.clone();
        if task_index < new_parts.len() {
            new_parts[task_index] = best_match;
        }
        new_parts.join(" ")
    } else {
        script_parts.join(" ")
    }
}

fn find_best_match<'a>(wrong_task: &str, tasks: &'a [String]) -> Option<&'a String> {
    // Simple fuzzy matching using prefix/similarity
    let wrong_lower = wrong_task.to_lowercase();
    
    tasks.iter()
        .filter(|t| t.to_lowercase().starts_with(&wrong_lower) || wrong_lower.starts_with(&t.to_lowercase()))
        .min_by_key(|t| {
            let t_lower = t.to_lowercase();
            let wrong_chars: Vec<char> = wrong_lower.chars().collect();
            let task_chars: Vec<char> = t_lower.chars().collect();
            
            // Simple Levenshtein-like distance
            let common_prefix = wrong_chars.iter()
                .zip(task_chars.iter())
                .take_while(|(a, b)| a == b)
                .count();
            
            wrong_task.len() + t.len() - 2 * common_prefix
        })
}