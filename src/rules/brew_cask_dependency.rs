use super::{Command, Rule};

pub struct BrewCask;

impl Rule for BrewCask {
    fn match(&self, command: &Command) -> bool {
        command.script_parts.contains(&"install".to_string())
            && command.output.contains("brew cask install")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let cask_install_lines: Vec<String> = command
            .output
            .split('\n')
            .map(|line| line.trim().to_string())
            .filter(|line| line.starts_with("brew cask install"))
            .collect();

        if cask_install_lines.is_empty() {
            return vec![];
        }

        let brew_cask_script = if cask_install_lines.len() > 1 {
            cask_install_lines.join(" && ")
        } else {
            cask_install_lines[0].