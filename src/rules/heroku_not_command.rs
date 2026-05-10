use super::{Command, Rule};

pub struct HerokuNotCommand;

impl Rule for HerokuNotCommand {
    fn name(&self) -> &str { "heroku_not_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("heroku") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
