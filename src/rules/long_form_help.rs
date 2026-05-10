use super::{Command, Rule};
use regex::Regex;

pub struct LongFormHelp;

impl Rule for LongFormHelp {
    fn name(&self) -> &'static str {
        "long_form_help"
    }

    fn matches(&self, cmd: &Command) -> bool {
        let help_regex = Regex::new(r"(?i)(?:Run|Try) '([^']+)'(?: or '[^']+')? for (?:details|more information).").unwrap();
        
        if help_regex.is_match(&cmd.output) {
            return true;
        }

        if cmd.output.contains("--help") {
            return true;
        }

        false
    }

    fn fix(&self, cmd: &Command) -> String {
        let help_regex = Regex::new(r"(?i)(?:Run|Try) '([^']+)'(?: or '[^']+')? for (?:details|more information).").unwrap();
        
        if let Some(captures) = help_regex.captures(&cmd.output) {
            return captures.get(1).map_or_else(|| cmd.script.clone(), |m| m.as_str().to_string());
        }

        cmd.script.replace("-h", "--help")
    }
}