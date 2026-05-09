use super::{Command, Rule};

pub struct BrewUpdateToUpgrade;

impl Rule for BrewUpdateToUpgrade {
    fn match_rule(&self, command: &Command) -> bool {
        let script_parts: Vec<&str> = command.script.split_whitespace().collect();
        if script_parts.len() < 2 || script_parts[0] != "brew" {
            return false;
        }
        
        command.script.contains("update")
            && command.output.contains("Error: This command updates brew itself")
            && command.output.contains("Use `brew upgrade")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec![command.script.replace("update", "upgrade")]
    }
}
