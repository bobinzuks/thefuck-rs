use super::{Command, Rule};
use regex::Regex;

pub struct ComposerRule;

impl Rule for ComposerRule {
    fn match(&self, command: &Command) -> bool {
        let output_lower = command.output.to_lowercase();
        
        (output_lower.contains("did you mean this?")
            || output_lower.contains("did you mean one of these?"))
            || (command.script_parts.contains(&"install".to_string())
                && output_lower.contains("composer require"))
    }

    fn get_new_command(&self, command: &Command) -> String {
        let output_lower = command.output.to_lowercase();
        
        let (broken_cmd, new_cmd) = if command.script_parts.contains(&"install".to_string())
            && output_lower.contains("composer require")
        {
            ("install".to_string()