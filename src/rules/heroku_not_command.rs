use super::{Command, Rule};
use regex::Regex;

pub struct HerokuNotCommand;

impl Rule for HerokuNotCommand {
    fn name() -> &'static str {
        "heroku_not_command"
    }

    fn matches(command: &Command) -> bool {
        command.output.contains("Run heroku _ to run")
    }

    fn fix(command: &Command) -> String {
        let re = Regex::new(r"Run heroku _ to run ([^.]*)").unwrap();
        if let Some(caps) = re.captures(&command.output) {
            caps.get(1).map_or(String::new(), |m| m.as_str().to_string())
        } else {
            String::new()
        }
    }
}