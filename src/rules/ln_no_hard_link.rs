use super::{Command, Rule};

pub struct LnNoHardLink;

impl Rule for LnNoHardLink {
    fn name(&self) -> &str { "ln_no_hard_link" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("ln") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
