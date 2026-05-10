use super::{Command, Rule};

pub struct FixAltSpace;

impl Rule for FixAltSpace {
    fn name(&self) -> &str { "fix_alt_space" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("fix") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
