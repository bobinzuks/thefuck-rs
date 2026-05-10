use super::{Command, Rule};

pub struct QuotationMarks;

impl Rule for QuotationMarks {
    fn name(&self) -> &str { "quotation_marks" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("quotation") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
