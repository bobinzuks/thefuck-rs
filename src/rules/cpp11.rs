use super::{Command, Rule};

pub struct Cpp11;

impl Rule for Cpp11 {
    fn name(&self) -> &str { "cpp11" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("cpp11") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
