use super::{Command, Rule};

pub struct WrongHyphenBeforeSubcommand;

impl Rule for WrongHyphenBeforeSubcommand {
    fn name(&self) -> &str { "wrong_hyphen_before_subcommand" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("wrong") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
