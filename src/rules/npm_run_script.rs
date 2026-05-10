use super::{Command, Rule};

pub struct NpmRunScript;

impl Rule for NpmRunScript {
    fn name(&self) -> &str { "npm_run_script" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("npm") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
