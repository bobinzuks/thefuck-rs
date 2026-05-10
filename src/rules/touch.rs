use super::{Command, Rule};
use regex::Regex;

pub struct Touch;

impl Rule for Touch {
    fn name(&self) -> &'static str {
        "touch"
    }

    fn matches(&self, command: &Command) -> bool {
        command.output.contains("No such file or directory")
    }

    fn fix(&self, command: &Command) -> String {
        let re = Regex::new(r"touch: (?:cannot touch ')?(.+)/.+'?:").unwrap();
        let path = re.captures(&command.output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");
        
        format!("mkdir -p {} && {}", path, command.script)
    }
}