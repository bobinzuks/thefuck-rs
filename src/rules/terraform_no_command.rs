use super::{Command, Rule};

pub struct TerraformNoCommand;

impl Rule for TerraformNoCommand {
    fn name(&self) -> &str { "terraform_no_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("terraform") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
