use super::{Command, Rule};

pub struct PythonCommand;

impl Rule for PythonCommand {
    fn name(&self) -> &str { "python_command" }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.starts_with("python ") &&
        cmd.output.contains("No such file or directory")
    }

    fn fix(&self, cmd: &Command) -> String {
        cmd.text.replacen("python ", "python3 ", 1)
    }
}
