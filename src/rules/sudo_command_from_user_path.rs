use super::{Command, Rule};
use regex::Regex;
use std::process::Command as ProcessCommand;

pub struct SudoCommandFromUserPath;

impl Rule for SudoCommandFromUserPath {
    fn name(&self) -> &'static str {
        "sudo_command_from_user_path"
    }

    fn matches(&self, cmd: &Command) -> bool {
        if !cmd.text.starts_with("sudo ") {
            return false;
        }
        
        if !cmd.output.contains("command not found") {
            return false;
        }

        let command_name = get_command_name(cmd);
        match command_name {
            Some(name) => {
                // Check if command exists in user's PATH
                ProcessCommand::new("which")
                    .arg(&name)
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
            }
            None => false,
        }
    }

    fn fix(&self, cmd: &Command) -> String {
        if let Some(command_name) = get_command_name(cmd) {
            cmd.text.replace(
                &command_name,
                &format!("env \"PATH=$PATH\" {}", command_name),
            )
        } else {
            cmd.text.clone()
        }
    }
}

fn get_command_name(command: &Command) -> Option<String> {
    let re = Regex::new(r"sudo: (.*): command not found").ok()?;
    re.captures(&command.output)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}