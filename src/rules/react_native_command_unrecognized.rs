use super::{Command, Rule};
use regex::Regex;

pub struct ReactNativeCommandUnrecognized;

impl Rule for ReactNativeCommandUnrecognized {
    fn name(&self) -> &'static str {
        "react_native_command_unrecognized"
    }

    fn matches(&self, cmd: &Command) -> bool {
        let re = Regex::new(r"Unrecognized command '.*'").unwrap();
        re.is_match(&cmd.output)
    }

    fn fix(&self, cmd: &Command) -> String {
        let re = Regex::new(r"Unrecognized command '(.*)'").unwrap();
        let misspelled = re.captures(&cmd.output)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        // Get available commands from help output
        let output = std::process::Command::new("react-native")
            .arg("--help")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default();

        let mut commands = Vec::new();
        let mut should_yield = false;

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.contains("Commands:") {
                should_yield = true;
                continue;
            }
            if should_yield {
                if let Some(cmd_name) = line.split_whitespace().next() {
                    commands.push(cmd_name.to_string());
                }
            }
        }

        // Replace command with closest match
        if commands.is_empty() || misspelled.is_empty() {
            return String::new();
        }

        let best_match = commands.iter()
            .map(|c| (c, strsim::levenshtein(c, misspelled)))
            .min_by_key(|&(_, dist)| dist)
            .map(|(c, _)| c.clone())
            .unwrap_or_default();

        cmd.text.replace(misspelled, &best_match)
    }
}