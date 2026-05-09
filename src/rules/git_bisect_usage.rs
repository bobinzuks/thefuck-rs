use super::{Command, Rule};
use regex::Regex;

pub struct GitBisect;

impl Rule for GitBisect {
    fn is_match(&self, command: &Command) -> bool {
        let has_bisect = command.script_parts.iter().any(|part| part == "bisect");
        let has_usage = command.output.contains("usage: git bisect");
        has_bisect && has_usage
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let broken_re = Regex::new(r"git bisect ([^ $]*).*").unwrap();
        let usage_re = Regex::new(r"usage: git bisect \[([^\]]+)\]").unwrap();

        let broken = broken_re.captures(&command.script)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        let usage = usage_re.captures(&command.output)
            .and_then(|caps