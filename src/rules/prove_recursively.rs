use super::{Command, Rule};
use std::path::Path;

pub struct ProveRecursively;

impl Rule for ProveRecursively {
    fn name(&self) -> &'static str {
        "prove_recursively"
    }

    fn matches(&self, cmd: &Command) -> bool {
        fn is_recursive(part: &str) -> bool {
            if part == "--recurse" {
                return true;
            }
            if !part.starts_with("--") && part.starts_with('-') && part.contains('r') {
                return true;
            }
            false
        }

        fn is_dir(part: &str) -> bool {
            !part.starts_with('-') && Path::new(part).is_dir()
        }

        if !cmd.output.contains("NOTESTS") {
            return false;
        }

        let parts: Vec<&str> = cmd.text.split_whitespace().collect();
        let has_recursive = parts[1..].iter().any(|part| is_recursive(part));
        if has_recursive {
            return false;
        }

        parts[1..].iter().any(|part| is_dir(part))
    }

    fn fix(&self, cmd: &Command) -> String {
        let parts: Vec<&str> = cmd.text.split_whitespace().collect();
        let mut new_parts: Vec<&str> = Vec::with_capacity(parts.len() + 1);
        new_parts.push(parts[0]);
        new_parts.push("-r");
        new_parts.extend_from_slice(&parts[1..]);
        new_parts.join(" ")
    }
}