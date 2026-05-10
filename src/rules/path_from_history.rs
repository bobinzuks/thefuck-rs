use super::{Command, Rule};
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

pub struct PathFromHistory;

impl Rule for PathFromHistory {
    fn name(&self) -> &'static str {
        "path_from_history"
    }

    fn matches(&self, command: &Command) -> bool {
        get_destination(command).is_some()
    }

    fn fix(&self, command: &Command) -> String {
        let destination = get_destination(command).unwrap();
        // Get valid history without current command (simplified - would need actual history)
        get_absolute_paths_from_history(command)
            .into_iter()
            .find(|path| path.contains(&destination))
            .unwrap_or_else(|| destination.to_string())
    }
}

fn get_destination(command: &Command) -> Option<String> {
    let patterns = [
        r"no such file or directory: (.*)$",
        r"cannot access '(.*)': No such file or directory",
        r": (.*): No such file or directory",
        r"can't cd to (.*)$",
    ];

    for pattern in &patterns {
        let re = Regex::new(pattern).ok()?;
        if let Some(caps) = re.captures(&command.output) {
            if let Some(matched) = caps.get(1) {
                let dest = matched.as_str().to_string();
                if command.script_parts.contains(&dest) {
                    return Some(dest);
                }
            }
        }
    }
    None
}

fn get_absolute_paths_from_history(command: &Command) -> Vec<String> {
    // Simplified: would normally iterate through shell history
    // For this implementation, we return an empty vec since we don't have history access
    Vec::new()
}