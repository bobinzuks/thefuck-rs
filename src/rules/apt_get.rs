use super::{Command, Rule};

pub struct AptGet;

impl Rule for AptGet {
    fn name(&self) -> &str { "apt_get" }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("command not found") &&
        (cmd.text.starts_with("apt-get") || cmd.text.starts_with("apt"))
    }

    fn fix(&self, cmd: &Command) -> String {
        let pkg = cmd.text.split_whitespace().last().unwrap_or("");
        format!("sudo apt-get install {}", pkg)
    }
}
