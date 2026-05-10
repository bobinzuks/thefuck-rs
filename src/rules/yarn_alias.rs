use super::{Command, Rule};
use regex::Regex;

pub struct YarnAlias;

impl Rule for YarnAlias {
    fn name(&self) -> &'static str {
        "yarn_alias"
    }

    fn matches(&self, cmd: &Command) -> bool {
        if cmd.text.split_whitespace().next() != Some("yarn") {
            return false;
        }
        cmd.output.contains("Did you mean")
    }

    fn fix(&self, cmd: &Command) -> String {
        let parts: Vec<&str> = cmd.text.split_whitespace().collect();
        let broken = parts[1];
        
        let re = Regex::new(r#"Did you mean [`"](?:yarn )?([^`"]*)[`"]"#).unwrap();
        let fix = re.captures(&cmd.output)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");
        
        cmd.text.replacen(broken, fix, 1)
    }
}