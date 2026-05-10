use super::{Command, Rule};
use regex::Regex;

pub struct PacmanInvalidOption;

impl Rule for PacmanInvalidOption {
    fn name(&self) -> &'static str {
        "pacman_invalid_option"
    }

    fn matches(&self, cmd: &Command) -> bool {
        if !cmd.output.starts_with("error: invalid option '-") {
            return false;
        }
        
        let options = ['s', 'u', 'r', 'q', 'f', 'd', 'v', 't'];
        options.iter().any(|&opt| {
            let pattern = format!(" -{}", opt);
            cmd.text.contains(&pattern)
        })
    }

    fn fix(&self, cmd: &Command) -> String {
        let re = Regex::new(r" -[dfqrstuv]").unwrap();
        let option = re.find(&cmd.text).unwrap().as_str().to_string();
        cmd.text.replacen(&option, &option.to_uppercase(), 1)
    }
}