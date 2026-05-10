use super::{Command, Rule};

pub struct GitDiffStaged;

impl Rule for GitDiffStaged {
    fn name(&self) -> &'static str {
        "git_diff_staged"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("diff") && !cmd.text.contains("--staged")
    }

    fn fix(&self, cmd: &Command) -> String {
        cmd.text.replacen("diff", "diff --staged", 1)
    }
}