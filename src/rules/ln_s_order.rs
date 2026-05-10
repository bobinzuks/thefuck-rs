use super::{Command, Rule};

pub struct LnSOrder;

impl Rule for LnSOrder {
    fn name(&self) -> &str { "ln_s_order" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("ln") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
