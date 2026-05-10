use super::{Command, Rule};

pub struct MissingSpaceBeforeSubcommand;

impl Rule for MissingSpaceBeforeSubcommand {
    fn name(&self) -> &str { "missing_space_before_subcommand" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("missing") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
