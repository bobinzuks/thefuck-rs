use super::{Command, Rule};

pub struct Unsudo;

impl Rule for Unsudo {
    fn name() -> &'static str {
        "unsudo"
    }

    fn matches(cmd: &Command) -> bool {
        // Check if the first part is 'sudo'
        if let Some(first) = cmd.text.split_whitespace().next() {
            if first != "sudo" {
                return false;
            }
        } else {
            return false;
        }

        let output_lower = cmd.output.to_lowercase();
        let patterns = ["you cannot perform this operation as root"];
        
        patterns.iter().any(|pattern| output_lower.contains(pattern))
    }

    fn fix(cmd: &Command) -> String {
        let parts: Vec<&str> = cmd.text.split_whitespace().collect();
        parts[1..].join(" ")
    }
}