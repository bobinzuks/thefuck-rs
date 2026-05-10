use super::{Command, Rule};
use regex::Regex;

pub struct TerraformNoCommand;

impl Rule for TerraformNoCommand {
    fn name(&self) -> &'static str {
        "terraform_no_command"
    }

    fn matches(&self, command: &Command) -> bool {
        let mistake_re = Regex::new(r#"(?<=Terraform has no command named ")([^"]+)(?="\.)"#).unwrap();
        let fix_re = Regex::new(r#"(?<=Did you mean ")([^"]+)(?="\?)"#).unwrap();
        
        command.app.as_deref() == Some("terraform") && 
        mistake_re.is_match(&command.output) && 
        fix_re.is_match(&command.output)
    }

    fn fix(&self, command: &Command) -> String {
        let mistake_re = Regex::new(r#"(?<=Terraform has no command named ")([^"]+)(?="\.)"#).unwrap();
        let fix_re = Regex::new(r#"(?<=Did you mean ")([^"]+)(?="\?)"#).unwrap();
        
        let mistake = mistake_re.find(&command.output).unwrap().as_str();
        let fix = fix_re.find(&command.output).unwrap().as_str();
        
        command.script.replace(mistake, fix)
    }
}