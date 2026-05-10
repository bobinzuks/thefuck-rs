use super::{Command, Rule};
use regex::Regex;

pub struct GitMerge;

impl Rule for GitMerge {
    fn name(&self) -> &'static str {
        "git_merge"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("merge")
            && cmd.output.contains(" - not something we can merge")
            && cmd.output.contains("Did you mean this?")
    }

    fn fix(&self, cmd: &Command) -> String {
        let unknown_branch_re = Regex::new(r"merge: (.+) - not something we can merge").unwrap();
        let remote_branch_re = Regex::new(r"Did you mean this\?\n\t([^\n]+)").unwrap();

        let unknown_branch = unknown_branch_re
            .captures(&cmd.output)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        let remote_branch = remote_branch_re
            .captures(&cmd.output)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        cmd.text.replacen(unknown_branch, remote_branch, 1)
    }
}