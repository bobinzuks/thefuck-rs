use super::{Command, Rule};

pub struct BrewUpdateFormula;

impl Rule for BrewUpdateFormula {
    fn name(&self) -> &str { "brew_update_formula" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("brew") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
