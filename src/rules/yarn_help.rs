use super::{Command, Rule};

pub struct YarnHelp;

impl Rule for YarnHelp {
    fn name(&self) -> &'static str {
        "yarn_help"
    }

    fn matches(&self, command: &Command) -> bool {
        if command.text.len() < 2 {
            return false;
        }
        let parts: Vec<&str> = command.text.split_whitespace().collect();
        parts.len() >= 2
            && parts[1] == "help"
            && command.output.contains("for documentation about this command.")
    }

    fn fix(&self, command: &Command) -> String {
        let re = regex::Regex::new(
            r"Visit ([^ ]*) for documentation about this command.",
        )
        .unwrap();
        let url = re
            .captures(&command.output)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");
        format!("open {}", url)
    }
}