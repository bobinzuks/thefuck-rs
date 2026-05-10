use regex::Regex;
use super::{Command, Rule};

pub struct YarnCommandReplaced;

impl Rule for YarnCommandReplaced {
    fn name(&self) -> &'static str {
        "yarn_command_replaced"
    }

    fn matches(&self, command: &Command) -> bool {
        let re = Regex::new(r#"Run "(.*)" instead"#).unwrap();
        re.is_match(&command.output)
    }

    fn fix(&self, command: &Command) -> String {
        let re = Regex::new(r#"Run "(.*)" instead"#).unwrap();
        if let Some(caps) = re.captures(&command.output) {
            caps.get(1).map_or_else(|| String::new(), |m| m.as_str().to_string())
        } else {
            String::new()
        }
    }
}