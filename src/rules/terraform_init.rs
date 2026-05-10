use super::{Command, Rule};

pub struct TerraformInit;

impl Rule for TerraformInit {
    fn name(&self) -> &'static str {
        "terraform_init"
    }

    fn matches(&self, command: &Command) -> bool {
        let output_lower = command.output.to_lowercase();
        output_lower.contains("this module is not yet installed")
            || output_lower.contains("initialization required")
    }

    fn fix(&self, command: &Command) -> String {
        format!("terraform init && {}", command.text)
    }
}