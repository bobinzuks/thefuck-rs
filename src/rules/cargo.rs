use super::{Command, Rule};

pub struct Cargo;

impl Rule for Cargo {
    fn name(&self) -> &str { "cargo" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("cargo") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
