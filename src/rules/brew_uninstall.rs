use super::{Command, Rule};

pub struct BrewUninstallForce;

impl Rule for BrewUninstallForce {
    fn match(&self, command: &Command) -> bool {
        if command.script_parts.len() < 2 {
            return false;
        }
        let subcommand = &command.script_parts[1];
        (subcommand == "uninstall" || subcommand == "rm" || subcommand == "remove")
            && command.output.contains("brew uninstall --force")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let mut command_parts = command.script_parts.clone();
        command_parts[1] = "uninstall".to_string();
        command_parts.insert(2, "--force".to_string());
        vec![command_parts.join(" ")]
    }
}
