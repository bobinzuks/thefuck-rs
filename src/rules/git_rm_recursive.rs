use super::{Command, Rule};

pub struct GitRmRecursive;

impl Rule for GitRmRecursive {
    fn name(&self) -> &'static str {
        "git_rm_recursive"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains(" rm ")
            && cmd.output.contains("fatal: not removing '")
            && cmd.output.contains("' recursively without -r")
    }

    fn fix(&self, cmd: &Command) -> String {
        let mut parts: Vec<&str> = cmd.text.split(' ').collect();
        if let Some(index) = parts.iter().position(|&s| s == "rm") {
            parts.insert(index + 1, "-r");
        }
        parts.join(" ")
    }
}