use super::{Command, Rule};

pub struct SwitchLang;

impl Rule for SwitchLang {
    fn name(&self) -> &str { "switch_lang" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("switch") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
