use super::{Command, Rule};

pub struct IfconfigDeviceNotFound;

impl Rule for IfconfigDeviceNotFound {
    fn name(&self) -> &str { "ifconfig_device_not_found" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("ifconfig") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
