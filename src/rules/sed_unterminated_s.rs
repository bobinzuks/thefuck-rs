use super::{Command, Rule};

pub struct SedUnterminatedS;

impl Rule for SedUnterminatedS {
    fn name(&self) -> &str { "sed_unterminated_s" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("sed") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
