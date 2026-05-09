use super::{Command, Rule};
use regex::Regex;

const INVALID_CHOICE: &str = "(?=az)(?:.*): '(.*)' is not in the '.*' command group.";
const OPTIONS: &str = "^The most similar choice to '.*' is:\n\\s*(.*)$";

pub struct AzRule;

impl Rule for AzRule {
    fn name(&self) -> &'static str {
        "az"
    }

    fn match(&self, command: &Command) -> bool {
        command.output.contains("is not in the")
            && command.output.contains("command group")
            && command.script.starts_with("az ")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let invalid_choice_re = Regex::new(INVALID_CHOICE).unwrap();
        let options_re = Regex::new(OPTIONS).unwrap();

        let mistake = invalid_choice_re
            .captures(&command.output)
            