use super::{Command, Rule};

pub struct FixFile;

impl Rule for FixFile {
    fn name(&self) -> &str { "fix_file" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("fix") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
