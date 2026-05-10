use super::{Command, Rule};

pub struct Systemctl;

impl Rule for Systemctl {
    fn name(&self) -> &'static str {
        "systemctl"
    }

    fn matches(&self, cmd: &Command) -> bool {
        let parts: Vec<&str> = cmd.text.split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }
        
        // Find index of "systemctl" in the command
        let sysctl_idx = match parts.iter().position(|&s| s == "systemctl") {
            Some(idx) => idx,
            None => return false,
        };
        
        // Check for exactly 3 words after "systemctl" and "Unknown operation" in output
        parts.len() - sysctl_idx == 3 
            && cmd.output.contains("Unknown operation '")
    }

    fn fix(&self, cmd: &Command) -> String {
        let mut parts: Vec<&str> = cmd.text.split_whitespace().collect();
        let len = parts.len();
        parts.swap(len - 1, len - 2);
        parts.join(" ")
    }
}