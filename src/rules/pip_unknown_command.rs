use super::{Command, Rule};
use regex::Regex;

pub struct PipUnknownCommand;

impl Rule for PipUnknownCommand {
    fn name(&self) -> &'static str {
        "pip_unknown_command"
    }

    fn matches(&self, command: &Command) -> bool {
        let script_contains_pip = command.text.contains("pip");
        let output_has_unknown = command.output.contains("unknown command");
        let output_has_suggestion = command.output.contains("maybe you meant");

        script_contains_pip && output_has_unknown && output_has_suggestion
    }

    fn fix(&self, command: &Command) -> Option<String> {
        let re_broken = Regex::new(r#"ERROR: unknown command "([^"]+)""#).ok()?;
        let re_new = Regex::new(r#"maybe you meant "([^"]+)""#).ok()?;

        let broken_cmd = re_broken.captures(&command.output)?.get(1)?.as_str();
        let new_cmd = re_new.captures(&command.output)?.get(1)?.as_str();

        Some(command.text.replacen(broken_cmd, new_cmd, 1))
    }
}