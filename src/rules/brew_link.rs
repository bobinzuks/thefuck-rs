use super::{Command, Rule};

pub struct BrewLinkOverwrite;

impl Rule for BrewLinkOverwrite {
    fn name(&self) -> &'static str {
        "brew_link_overwrite"
    }

    fn match(&self, command: &Command) -> bool {
        if command.script_parts.len() < 2 {
            return false;
        }
        let cmd = command.script_parts[0].as_str();
        let subcmd = command.script_parts[1].as_str();
        
        cmd == "brew" 
            && (subcmd == "ln" || subcmd == "link")
            && command.output.contains("brew link --overwrite --dry-run")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let mut parts = command.script_parts.clone();
        parts[1] = "link".to_string();
        parts.insert(2, "--overwrite".to_string());
        parts.insert