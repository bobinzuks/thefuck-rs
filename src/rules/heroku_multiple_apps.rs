use super::{Command, Rule};

pub struct HerokuMultipleApps;

impl Rule for HerokuMultipleApps {
    fn name(&self) -> &str { "heroku_multiple_apps" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("heroku") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
