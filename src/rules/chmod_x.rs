use super::{Command, Rule};

pub struct ChmodX;

impl Rule for ChmodX {
    fn name(&self) -> &str { "chmod_x" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("chmod") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
