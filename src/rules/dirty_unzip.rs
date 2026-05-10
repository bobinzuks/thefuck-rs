use super::{Command, Rule};

pub struct DirtyUnzip;

impl Rule for DirtyUnzip {
    fn name(&self) -> &str { "dirty_unzip" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("dirty") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
