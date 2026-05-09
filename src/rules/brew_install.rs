use super::{Command, Rule};
use std::process::Command as ProcessCommand;

pub struct BrewInstallSuggestion;

impl Rule for BrewInstallSuggestion {
    fn name(&self) -> &'static str {
        "brew_install_suggestion"
    }

    fn enabled_by_default(&self) -> bool {
        brew_available()
    }

    fn match(&self, command: &Command) -> bool {
        command.script.contains("install")
            && command.output.contains("No available formula")
            && command.output.contains("Did you mean")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let re = regex::Regex::new(
            r#"Warning: No available formula with the name "(?:[^"]+)". Did you mean (.+)\?"#
        ).unwrap();
        
        if let Some(captures) = re.captures(&command.outpu