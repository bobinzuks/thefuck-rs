use super::{Command, Rule};

pub struct CpCreateDestination;

impl Rule for CpCreateDestination {
    fn name(&self) -> &str { "cp_create_destination" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("cp") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
