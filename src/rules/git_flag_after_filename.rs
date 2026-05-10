use super::{Command, Rule};

pub struct GitFlagAfterFilename;

impl Rule for GitFlagAfterFilename {
    fn name(&self) -> &str { "git_flag_after_filename" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
