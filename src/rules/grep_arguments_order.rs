use super::{Command, Rule};
use std::path::Path;

pub struct GrepArgumentsOrder;

impl Rule for GrepArgumentsOrder {
    fn name() -> &'static str {
        "grep_arguments_order"
    }

    fn matches(cmd: &Command) -> bool {
        if cmd.text.is_empty() || cmd.output.is_empty() {
            return false;
        }

        let parts: Vec<&str> = cmd.text.split_whitespace().collect();
        if parts.len() < 2 {
            return false;
        }

        if !cmd.output.contains(": No such file or directory") {
            return false;
        }

        let cmd_name = parts[0];
        if cmd_name != "grep" && cmd_name != "egrep" {
            return false;
        }

        _get_actual_file(&parts).is_some()
    }

    fn fix(cmd: &Command) -> String {
        let parts: Vec<&str> = cmd.text.split_whitespace().collect();
        let actual_file = _get_actual_file(&parts).unwrap();

        let mut new_parts: Vec<&str> = parts.clone();
        if let Some(pos) = new_parts.iter().position(|&p| p == actual_file) {
            new_parts.remove(pos);
        }
        new_parts.push(actual_file);

        new_parts.join(" ")
    }
}

fn _get_actual_file(parts: &[&str]) -> Option<&str> {
    for part in parts.iter().skip(1) {
        if Path::new(part).is_file() || Path::new(part).is_dir() {
            return Some(part);
        }
    }
    None
}