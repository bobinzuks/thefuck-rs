use super::{Command, Rule};

pub struct LsLah;

impl Rule for LsLah {
    fn name(&self) -> &'static str {
        "ls_lah"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("ls") && !cmd.text.contains("ls -")
    }

    fn fix(&self, cmd: &Command) -> String {
        let parts: Vec<&str> = cmd.text.split_whitespace().collect();
        let mut new_parts = parts.clone();
        if let Some(pos) = new_parts.iter().position(|&s| s == "ls") {
            new_parts[pos] = "ls -lah";
        }
        new_parts.join(" ")
    }
}