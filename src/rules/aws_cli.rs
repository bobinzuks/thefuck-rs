use super::{Command, Rule};
use regex::Regex;

pub struct AwsInvalidChoice;

impl Rule for AwsInvalidChoice {
    fn name(&self) -> &'static str {
        "aws_invalid_choice"
    }

    fn match(&self, command: &Command) -> bool {
        let output = command.output.to_lowercase();
        output.contains("usage:") && output.contains("maybe you meant:")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let invalid_choice_re = Regex::new(r"(?<=Invalid choice: ')(.*)(?=', maybe you meant:)").unwrap();
        let options_re = Regex::new(r"^\s*\*\s(.*)").unwrap();

        let mistake = invalid_choice_re
            .captures(&command.output)
            .and_then(|caps| caps.get(0))
            .map(|m| m.as_str())
            .unwrap_or("");

        let opti