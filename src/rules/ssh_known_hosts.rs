use super::{Command, Rule};
use regex::Regex;

pub struct SshKnownHosts;

impl Rule for SshKnownHosts {
    fn name(&self) -> &'static str {
        "ssh_known_hosts"
    }

    fn matches(&self, cmd: &Command) -> bool {
        if cmd.text.is_empty() {
            return false;
        }

        let commands = ["ssh", "scp"];
        if !commands.iter().any(|c| cmd.text.starts_with(c)) {
            return false;
        }

        let patterns = [
            r"WARNING: REMOTE HOST IDENTIFICATION HAS CHANGED!",
            r"WARNING: POSSIBLE DNS SPOOFING DETECTED!",
            r"Warning: the \S+ host key for '([^']+)' differs from the key for the IP address '([^']+)'",
        ];

        patterns.iter().any(|pattern| {
            Regex::new(pattern).unwrap().is_match(&cmd.output)
        })
    }

    fn fix(&self, cmd: &Command) -> String {
        cmd.text.clone()
    }

    fn side_effect(&self, old_cmd: &Command, new_cmd: &Command) {
        let offending_pattern = Regex::new(
            r"(?:Offending (?:key for IP|\S+ key)|Matching host key) in ([^:]+):(\d+)"
        ).unwrap();

        for cap in offending_pattern.captures_iter(&old_cmd.output) {
            let filepath = cap.get(1).unwrap().as_str().to_string();
            let lineno: usize = cap.get(2).unwrap().as_str().parse().unwrap();

            let content = std::fs::read_to_string(&filepath).unwrap();
            let mut lines: Vec<&str> = content.lines().collect();
            if lineno > 0 && lineno <= lines.len() {
                lines.remove(lineno - 1);
            }
            std::fs::write(&filepath, lines.join("\n")).unwrap();
        }
    }
}