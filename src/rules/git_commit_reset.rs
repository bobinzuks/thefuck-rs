use super::{Command, Rule};

pub struct GitCommitReset;

impl Rule for GitCommitReset {
    fn name(&self) -> &'static str {
        "git_commit_reset"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("commit") || cmd.output.contains("commit")
    }

    fn fix(&self, _cmd: &Command) -> String {
        "git reset HEAD~".to_string()
    }
}