use super::{Command, Rule};
use std::process::Command as StdCommand;
use std::collections::HashSet;

pub struct GulpNotTask;

impl Rule for GulpNotTask {
    fn name(&self) -> &'static str {
        "gulp_not_task"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("is not in your gulpfile")
    }

    fn fix(&self, cmd: &Command) -> String {
        let wrong_task = {
            let re = regex::Regex::new(r"Task '(\w+)' is not in your gulpfile").unwrap();
            let caps = re.captures(&cmd.output).unwrap();
            caps.get(1).unwrap().as_str().to_string()
        };

        let tasks = get_gulp_tasks();
        replace_command(cmd, &wrong_task, &tasks)
    }
}

fn get_gulp_tasks() -> Vec<String> {
    let output = StdCommand::new("gulp")
        .arg("--tasks-simple")
        .output()
        .expect("Failed to execute gulp --tasks-simple");

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

fn replace_command(cmd: &Command, wrong_task: &str, tasks: &[String]) -> String {
    let fuzzy_tasks = tasks
        .iter()
        .filter(|task| {
            let distance = strsim::levenshtein(wrong_task, task);
            distance <= 2 || task.starts_with(wrong_task) || wrong_task.starts_with(task.as_str())
        })
        .collect::<Vec<_>>();

    if fuzzy_tasks.len() == 1 {
        cmd.text.replace(wrong_task, fuzzy_tasks[0])
    } else if fuzzy_tasks.len() > 1 {
        format!("{} [{}]", cmd.text, fuzzy_tasks.join(", "))
    } else {
        cmd.text.clone()
    }
}