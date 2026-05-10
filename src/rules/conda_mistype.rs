use super::{Command, Rule};

pub struct CondaMistype;

impl Rule for CondaMistype {
    fn name(&self) -> &str { "conda_mistype" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("conda") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
