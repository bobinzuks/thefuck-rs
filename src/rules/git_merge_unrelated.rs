use super::{Command, Rule};

pub struct GitMergeUnrelated;

impl Rule for GitMergeUnrelated {
    fn name(&self) -> &str { "git_merge_unrelated" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
