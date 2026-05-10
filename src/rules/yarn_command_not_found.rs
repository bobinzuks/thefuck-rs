use super::{Command, Rule};
use regex::Regex;
use std::process::Command as ProcessCommand;
use std::sync::OnceLock;

static REGEX: OnceLock<Regex> = OnceLock::new();

fn get_regex() -> &'static Regex {
    REGEX.get_or_init(|| Regex::new(r#"error Command "(.*)" not found."#).unwrap())
}

static YARN_PATH: OnceLock<Option<String>> = OnceLock::new();

fn get_yarn_path() -> Option<&'static str> {
    YARN_PATH
        .get_or_init(|| {
            // Check if yarn is available
            match ProcessCommand::new("which").arg("yarn").output() {
                Ok(output) if output.status.success() => {
                    String::from_utf8(output.stdout)
                        .ok()
                        .map(|s| s.trim().to_string())
                }
                _ => None,
            }
        })
        .as_deref()
}

fn get_all_tasks() -> Vec<String> {
    let yarn_path = match get_yarn_path() {
        Some(path) => path,
        None => return Vec::new(),
    };

    // Try to use cached result
    static CACHED_TASKS: OnceLock<Vec<String>> = OnceLock::new();
    if let Some(tasks) = CACHED_TASKS.get() {
        return tasks.clone();
    }

    let output = ProcessCommand::new(yarn_path)
        .arg("--help")
        .output()
        .ok();

    let tasks = match output {
        Some(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut should_yield = false;
            let mut tasks = Vec::new();

            for line in stdout.lines() {
                let line = line.trim();
                if line == "Commands:" {
                    should_yield = true;
                    continue;
                }
                if should_yield && line.starts_with("- ") {
                    if let Some(task) = line.split(' ').last() {
                        tasks.push(task.to_string());
                    }
                }
            }
            tasks
        }
        _ => Vec::new(),
    };

    let _ = CACHED_TASKS.set(tasks.clone());
    tasks
}

fn npm_commands() -> Vec<(&'static str, &'static str)> {
    vec![("require", "add")]
}

pub struct YarnCommandNotFound;

impl Rule for YarnCommandNotFound {
    fn name() -> &'static str {
        "yarn_command_not_found"
    }

    fn matches(cmd: &Command) -> bool {
        cmd.text.starts_with("yarn") && get_regex().is_match(&cmd.output)
    }

    fn fix(cmd: &Command) -> Option<String> {
        let caps = get_regex().captures(&cmd.output)?;
        let misspelled_task = caps.get(1)?.as_str().to_string();

        // Check npm commands mapping
        for (npm_cmd, yarn_cmd) in npm_commands() {
            if misspelled_task == npm_cmd {
                return Some(cmd.text.replace(npm_cmd, yarn_cmd));
            }
        }

        // Get available tasks and find similar
        let tasks = get_all_tasks();
        if tasks.is_empty() {
            return None;
        }

        // Simple similarity check - find closest match
        let best_match = tasks
            .iter()
            .min_by_key(|task| {
                // Levenshtein distance approximation
                let misspelled = &misspelled_task;
                let task_bytes = task.as_bytes();
                let misspelled_bytes = misspelled.as_bytes();
                let m = task_bytes.len();
                let n = misspelled_bytes.len();
                
                if m == 0 || n == 0 {
                    return m.max(n);
                }
                
                // Simple Hamming-like distance for same length, else length difference
                if m == n {
                    task_bytes.iter().zip(misspelled_bytes).filter(|(a, b)| a != b).count()
                } else {
                    m.abs_diff(n) + 1
                }
            })?;

        Some(cmd.text.replace(&misspelled_task, best_match))
    }
}