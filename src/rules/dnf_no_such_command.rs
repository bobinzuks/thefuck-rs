use super::{Command, Rule};
use regex::Regex;
use std::process::Command as StdCommand;

fn parse_operations(help_text_lines: &str) -> Vec<String> {
    let operation_regex = Regex::new(r"^([a-z-]+) +").unwrap();
    operation_regex
        .captures_iter(help_text_lines)
        .map(|cap| cap[1].to_string())
        .collect()
}

fn get_operations() -> Vec<String> {
    let output = StdCommand::new("dnf")
        .arg("--help")
        .output()
        .expect("Failed to execute dnf --help");
    let help_text = String::from_utf8_lossy(&output.stdout);
    parse_operations(&help_text)
}

fn replace_command(command: &Command, misspelled: &str, operations: &[String]) -> Vec<String> {
    // Simple Levenshtein distance based replacement
    operations
        .iter()
        .filter(|op| {
