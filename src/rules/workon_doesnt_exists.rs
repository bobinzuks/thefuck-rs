use std::path::PathBuf;
use super::{Command, Rule};

fn get_all_environments() -> Vec<String> {
    let root = PathBuf::from("~/.virtualenvs");
    let root = if let Ok(expanded) = root.expanduser() {
        expanded
    } else {
        return vec![];
    };
    
    if !root.is_dir() {
        return vec![];
    }
    
    let mut envs = Vec::new();
    if let Ok(entries) = root.read_dir() {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    envs.push(name.to_string());
                }
            }
        }
    }
    envs
}

pub struct WorkonDoesntExists;

impl Rule for WorkonDoesntExists {
    fn name(&self) -> &str {
        "workon_doesnt_exist"
    }

    fn matches(&self, command: &Command) -> bool {
        let envs = get_all_environments();
        command.script_parts.len() >= 2 
            && !envs.contains(&command.script_parts[1])
    }

    fn fix(&self, command: &Command) -> String {
        let misspelled_env = &command.script_parts[1];
        let create_new = format!("mkvirtualenv {}", misspelled_env);
        
        let available = get_all_environments();
        if !available.is_empty() {
            let replacement = replace_command(command, misspelled_env, available);
            if let Some(first) = replacement.first() {
                format!("{}", first)
            } else {
                create_new
            }
        } else {
            create_new
        }
    }
}