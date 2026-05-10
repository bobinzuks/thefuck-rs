use super::{Command, Rule};

pub struct NpmWrongCommand;

impl Rule for NpmWrongCommand {
    fn name() -> &'static str {
        "npm_wrong_command"
    }

    fn matches(cmd: &Command) -> bool {
        if cmd.text.split_whitespace().next() != Some("npm") {
            return false;
        }
        if !cmd.output.contains("where <command> is one of:") {
            return false;
        }
        cmd.text
            .split_whitespace()
            .skip(1)
            .find(|part| !part.starts_with('-'))
            .is_some()
    }

    fn fix(cmd: &Command) -> String {
        let wrong_command = cmd
            .text
            .split_whitespace()
            .skip(1)
            .find(|part| !part.starts_with('-'))
            .unwrap_or("");

        let available_commands: Vec<&str> = cmd
            .output
            .lines()
            .skip_while(|line| !line.starts_with("where <command> is one of:"))
            .skip(1)
            .take_while(|line| !line.is_empty())
            .flat_map(|line| line.split(", "))
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let fixed = super::get_closest(wrong_command, &available_commands);
        cmd.text.replacen(wrong_command, &fixed, 1)
    }
}