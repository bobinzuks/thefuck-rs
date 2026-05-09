use super::{Command, Rule};
use regex::Regex;

pub struct CpDirectory;

impl Rule for CpDirectory {
    fn is_match(&self, command: &Command) -> bool {
        let output = command.output.to_lowercase();
        output.contains("omitting directory") || output.contains("is a directory")
    }

    fn apply(&self, command: &Command) -> Option<String> {
        let re = Regex::new(r"^cp").ok()?;
        Some(re.replace(&command.script, "cp -a").to_string())
    }

    fn tag(&self) -> &'static str {
        "cp_directory"
    }
}
