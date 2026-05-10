use super::{Command, Rule};

pub struct GitPullUncommittedChanges;

impl Rule for GitPullUncommittedChanges {
    fn name(&self) -> &'static str {
        "git_pull_uncommitted_changes"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("pull")
            && (cmd.output.contains("You have unstaged changes")
                || cmd.output.contains("contains uncommitted changes"))
    }

    fn fix(&self, _cmd: &Command) -> String {
        format!("git stash && git pull && git stash pop")
    }
}