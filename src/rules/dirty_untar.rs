use super::{Command, Rule};

pub struct DirtyUntar;

impl Rule for DirtyUntar {
    fn name(&self) -> &str { "dirty_untar" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("dirty") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
