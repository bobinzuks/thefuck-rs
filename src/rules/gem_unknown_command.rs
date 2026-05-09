use super::{Command, Rule};
use regex::Regex;
use std::process::Command as ProcessCommand;
use once_cell::sync::Lazy;

static GEM_CACHE: Lazy<Vec<String>> = Lazy::new(|| {
    if let Ok(output) = ProcessCommand::new("gem")
        .args(["help", "commands"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout
            .lines()
            .filter(|line| line.starts_with("    "))
            .filter_map(|line| line.trim().split(' ').next())
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    }
});

pub struct GemRule;

impl Rule for GemRule {
    fn is_match(&self, command: &Command) -> bool {
        command.output.contains("ERROR:  While executing gem ... (Gem::CommandLineError)")
            &&