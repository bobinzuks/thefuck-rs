use super::{Command, Rule};
use regex::Regex;
use std::process::{Command as ProcessCommand, Stdio};
use once_cell::sync::Lazy;

pub struct PortAlreadyInUse;

static PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"bind on address \('.*', (?P<port>\d+)\)").unwrap(),
        Regex::new(r"Unable to bind [^ ]*:(?P<port>\d+)").unwrap(),
        Regex::new(r"can't listen on port (?P<port>\d+)").unwrap(),
        Regex::new(r"listen EADDRINUSE [^ ]*:(?P<port>\d+)").unwrap(),
    ]
});

fn get_port(output: &str) -> Option<String> {
    for pattern in PATTERNS.iter() {
        if let Some(caps) = pattern.captures(output) {
            return Some(caps["port"].to_string());
        }
    }
    None
}

fn get_pid_by_port(port: &str) -> Option<String> {
    let output = ProcessCommand::new("lsof")
        .args(&["-i", &format!(":{}", port)])
        .stdout(Stdio::piped())
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() > 1 {
        lines[1].split_whitespace().nth(1).map(|s| s.to_string())
    } else {
        None
    }
}

impl Rule for PortAlreadyInUse {
    fn name(&self) -> &'static str {
        "port_already_in_use"
    }

    fn matches(&self, cmd: &Command) -> bool {
        if let Some(port) = get_port(&cmd.output) {
            get_pid_by_port(&port).is_some()
        } else {
            false
        }
    }

    fn fix(&self, cmd: &Command) -> String {
        let port = get_port(&cmd.output).unwrap();
        let pid = get_pid_by_port(&port).unwrap();
        format!("kill {} && {}", pid, cmd.text)
    }
}