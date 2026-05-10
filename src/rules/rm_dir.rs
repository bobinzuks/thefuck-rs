use super::{Command, Rule};
use regex::Regex;

pub struct RmDir;

impl Rule for RmDir {
    fn name(&self) -> &'static str {
        "rm_dir"
    }

    fn matches(&self, command: &Command) -> bool {
        command.text.contains("rm")
            && command.output.to_lowercase().contains("is a directory")
    }

    fn fix(&self, command: &Command) -> String {
        let arguments = if command.text.contains("hdfs") {
            "-r"
        } else {
            "-rf"
        };
        let re = Regex::new(r"\brm (.*)").unwrap();
        re.replace(&command.text, |caps: &regex::Captures| {
            format!("rm {} {}", arguments, &caps[1])
        })
        .to_string()
    }
}