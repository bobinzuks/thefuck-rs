use super::{Command, Rule};
use std::process::Command as ProcessCommand;

pub struct YumInvalidOperation;

impl Rule for YumInvalidOperation {
    fn name(&self) -> &'static str {
        "yum_invalid_operation"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("No such command: ")
    }

    fn fix(&self, cmd: &Command) -> String {
        let invalid_operation = &cmd.script_parts[1];

        if invalid_operation == "uninstall" {
            cmd.script.replace("uninstall", "remove")
        } else {
            let operations = get_operations();
            replace_command(cmd, invalid_operation, &operations)
        }
    }
}

fn get_operations() -> Vec<String> {
    if let Ok(output) = ProcessCommand::new("yum").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut lines: Vec<&str> = output_str.lines().collect();
        
        // Find "List of Commands:" and skip 2 lines after it
        let start_index = lines.iter().position(|l| l.starts_with("List of Commands:"));
        
        if let Some(idx) = start_index {
            lines = lines.into_iter().skip(idx + 2).collect();
            
            // Take while lines are not empty
            let mut operations = Vec::new();
            for line in lines {
                if line.trim().is_empty() {
                    break;
                }
                if let Some(op) = line.trim().split(' ').next() {
                    operations.push(op.to_string());
                }
            }
            return operations;
        }
    }
    Vec::new()
}

fn replace_command(cmd: &Command, invalid_operation: &str, operations: &[String]) -> String {
    // Simple replacement - find closest match or return original
    if let Some(best_match) = operations.iter().find(|op| {
        op.starts_with(invalid_operation) || invalid_operation.starts_with(op.as_str())
    }) {
        cmd.script.replace(invalid_operation, best_match)
    } else {
        cmd.script.clone()
    }
}