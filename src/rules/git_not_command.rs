use regex::Regex;
use super::{Command, Rule};

pub struct GitNotCommand;

impl Rule for GitNotCommand {
    fn name(&self) -> &'static str {
        "git_not_command"
    }

    fn matches(&self, command: &Command) -> bool {
        command.output.contains(" is not a git command. See 'git --help'.")
            && (command.output.contains("The most similar command")
                || command.output.contains("Did you mean"))
    }

    fn fix(&self, command: &Command) -> String {
        let re = Regex::new(r"git: '([^']*)' is not a git command").unwrap();
        let broken_cmd = re.captures(&command.output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        let matched = get_all_matched_commands(&command.output, &["The most similar command", "Did you mean"]);
        replace_command(command, broken_cmd, matched)
    }
}

fn get_all_matched_commands(output: &str, patterns: &[&str]) -> Vec<String> {
    let mut commands = Vec::new();
    for pattern in patterns {
        if output.contains(pattern) {
            let start = output.find(pattern).unwrap_or(0);
            let slice = &output[start..];
            for line in slice.lines().take(5) {
                if line.contains("git ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for part in parts {
                        if !part.contains("git") && !part.contains("'") {
                            commands.push(part.to_string());
                        }
                    }
                }
            }
        }
    }
    commands
}

fn replace_command(command: &Command, broken_cmd: &str, matched: Vec<String>) -> String {
    if matched.is_empty() {
        return command.text.clone();
    }
    command.text.replace(broken_cmd, &matched[0])
}