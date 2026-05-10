use super::{Command, Rule};

pub struct PortAlreadyInUse;

impl Rule for PortAlreadyInUse {
    fn name(&self) -> &str { "port_already_in_use" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("port") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
