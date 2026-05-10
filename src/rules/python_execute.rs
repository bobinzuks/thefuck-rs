use super::{Command, Rule};

pub struct PythonExecute;

impl Rule for PythonExecute {
    fn name(&self) -> &str { "python_execute" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("python") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
