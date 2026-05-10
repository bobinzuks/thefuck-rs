use super::{Command, Rule};
use regex::Regex;

pub struct MkdirP;

impl Rule for MkdirP {
    fn name(&self) -> &'static str {
        "mkdir_p"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("mkdir")
            && cmd.output.contains("No such file or directory")
    }

    fn fix(&self, cmd: &Command) -> String {
        let re = Regex::new(r"\bmkdir (.*)").unwrap();
        re.replace(&cmd.text, "mkdir -p $1").to_string()
    }
}