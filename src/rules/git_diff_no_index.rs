use super::{Command, Rule};

pub struct GitDiffNoIndex;

impl Rule for GitDiffNoIndex {
    fn name(&self) -> &str { "git_diff_no_index" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
