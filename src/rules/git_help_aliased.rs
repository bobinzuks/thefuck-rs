use super::{Command, Rule};

pub struct GitHelpAliased;

impl Rule for GitHelpAliased {
    fn name(&self) -> &str { "git_help_aliased" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
