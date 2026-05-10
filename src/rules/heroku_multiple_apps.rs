use super::{Command, Rule};
use regex::Regex;

pub struct HerokuMultipleApps;

impl Rule for HerokuMultipleApps {
    fn name() -> &'static str {
        "heroku_multiple_apps"
    }

    fn matches(cmd: &Command) -> bool {
        cmd.output.contains("https://devcenter.heroku.com/articles/multiple-environments")
    }

    fn fix(cmd: &Command) -> String {
        let re = Regex::new(r"([^ ]*) \([^)]*\)").unwrap();
        let apps: Vec<&str> = re
            .captures_iter(&cmd.output)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect();
        
        apps.join(" ")
    }
}