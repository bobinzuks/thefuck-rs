use super::{Command, Rule};

pub struct QuotationMarks;

impl Rule for QuotationMarks {
    fn name() -> &'static str {
        "quotation_marks"
    }

    fn matches(cmd: &Command) -> bool {
        cmd.text.contains('\'') && cmd.text.contains('\"')
    }

    fn fix(cmd: &Command) -> String {
        cmd.text.replace('\'', "\"")
    }
}