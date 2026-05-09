use super::{Command, Rule};

pub struct AptInvalidOperation;

impl Rule for AptInvalidOperation {
    fn is_match(&self, command: &Command) -> bool {
        command.output.contains("E: Invalid operation")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let invalid_operation = command.output.split_whitespace().last()
            .unwrap_or("")
            .to_string();
        
        if invalid_operation == "uninstall" {
            return vec![command.script.replace("uninstall", "remove")];
        }

        let operations = get_operations(&command.script_parts[0]);
        replace_command(&command.script, &invalid_operation, &operations)
    }
}

fn get_operations(app: &str) -> Vec<String> {
    let output = std::process::Command::new(app)
        .arg