use super::{Command, Rule};

pub struct GitCommitAmend;

impl Rule for GitCommitAmend {
    fn name() -> String {
        "git_commit_amend".to_string()
    }

    fn matches(cmd: &Command) -> bool {
        cmd.text.contains("commit") && cmd.output.contains("git commit")
    }

    fn fix(cmd: &Command) -> String {
        "git commit --amend".to_string()
    }
}