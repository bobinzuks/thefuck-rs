use super::{Command, Rule};
use regex::Regex;

pub struct GitBranch;

impl Rule for GitBranch {
    fn name(&self) -> &str { "git_branch" }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.starts_with("git") &&
        cmd.output.contains("most similar command")
    }

    fn fix(&self, cmd: &Command) -> String {
        let re = Regex::new(r"most similar command is\s+(\S+)").unwrap();
        if let Some(cap) = re.captures(&cmd.output) {
            return format!("git {}", &cap[1]);
        }
        cmd.text.clone()
    }
}
