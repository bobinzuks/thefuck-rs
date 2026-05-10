use super::{Command, Rule};
use regex::Regex;

pub struct RemoveShellPromptLiteral;

impl Rule for RemoveShellPromptLiteral {
    fn name(&self) -> &'static str {
        "remove_shell_prompt_literal"
    }

    fn matches(&self, command: &Command) -> bool {
        command.output.contains("$: command not found")
            && Regex::new(r"^[\s]*\$ [\S]+")
                .ok()
                .map_or(false, |re| re.is_match(&command.text))
    }

    fn fix(&self, command: &Command) -> String {
        command.text.trim_start_matches("$ ").to_string()
    }
}