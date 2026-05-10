use super::{Command, Rule};
use std::process::Command as StdCommand;
use regex::Regex;
use std::collections::HashMap;

pub struct OmnienvNoSuchCommand;

impl Rule for OmnienvNoSuchCommand {
    fn name(&self) -> &'static str {
        "omnienv_no_such_command"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("env: no such command ")
    }

    fn fix(&self, cmd: &Command) -> String {
        let re = Regex::new(r"env: no such command ['`]([^']*)'").unwrap();
        let broken = re.captures(&cmd.output)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        let common_typos: HashMap<&str, Vec<&str>> = [
            ("list", vec!["versions", "install --list"]),
            ("remove", vec!["uninstall"]),
        ].iter().cloned().collect();

        let app = cmd.script_parts.first().map(|s| s.as_str()).unwrap_or("");
        let mut matched: Vec<String> = Vec::new();

        if let Some(typos) = common_typos.get(broken) {
            for typo in typos {
                matched.push(cmd.script.replace(broken, typo));
            }
        }

        if let Ok(output) = StdCommand::new(app).arg("commands").output() {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                for line in stdout.lines() {
                    let command_name = line.trim();
                    if !command_name.is_empty() && command_name != broken {
                        matched.push(cmd.script.replace(broken, command_name));
                    }
                }
            }
        }

        matched.first().cloned().unwrap_or_default()
    }
}