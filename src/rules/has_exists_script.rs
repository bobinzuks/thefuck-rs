use super::{Command, Rule};

pub struct HasExistsScript;

impl Rule for HasExistsScript {
    fn name(&self) -> &str { "has_exists_script" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("has") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
