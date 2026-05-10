use super::{Command, Rule};

pub struct AdbUnknownCommand;

impl Rule for AdbUnknownCommand {
    fn name(&self) -> &str { "adb_unknown_command" }
    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.starts_with("adb") && cmd.output.contains("unknown command")
    }
    fn fix(&self, cmd: &Command) -> String {
        cmd.text.clone()
    }
}
