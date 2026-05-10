use super::{Command, Rule};

pub struct GitRmStaged;

impl Rule for GitRmStaged {
    fn name(&self) -> &str { "git_rm_staged" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
