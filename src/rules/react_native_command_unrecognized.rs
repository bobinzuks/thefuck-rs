use super::{Command, Rule};

pub struct ReactNativeCommandUnrecognized;

impl Rule for ReactNativeCommandUnrecognized {
    fn name(&self) -> &str { "react_native_command_unrecognized" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("react") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
