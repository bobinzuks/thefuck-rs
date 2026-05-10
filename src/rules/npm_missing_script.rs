use super::{Command, Rule};

pub struct NpmMissingScript;

impl Rule for NpmMissingScript {
    fn name(&self) -> &str { "npm_missing_script" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("npm") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
