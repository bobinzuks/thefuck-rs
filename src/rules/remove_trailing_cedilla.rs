use super::{Command, Rule};

pub struct RemoveTrailingCedilla;

impl Rule for RemoveTrailingCedilla {
    fn name(&self) -> &'static str {
        "remove_trailing_cedilla"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.ends_with('ç') || cmd.output.ends_with('ç')
    }

    fn fix(&self, cmd: &Command) -> String {
        let new_text = cmd.text.trim_end_matches('ç').to_string();
        let new_output = cmd.output.trim_end_matches('ç').to_string();
        format!("{}{}", new_text, new_output)
    }
}