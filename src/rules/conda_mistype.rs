use super::{Command, Rule};
use regex::Regex;

pub struct CondaRule;

impl Rule for CondaRule {
    fn name(&self) -> &'static str {
        "conda"
    }

    fn match_rule(&self, command: &Command) -> bool {
        command.output.contains("Did you mean 'conda")
    }

    fn get_new_command(&self, command: &Command) -> Option<Vec<String>> {
        let re = Regex::new(r"'conda ([^']*)'").ok()?;
        let matches: Vec<&str> = re.find_iter(&command.output)
            .filter_map(|m| {
                m.as_str().strip_prefix("'conda ")?.strip_suffix("'")
            })
            .collect();
        
        if matches.len() >= 2 {
            let broken_cmd = matches[0];
            let correct_cmd = matches[1];
            Some(vec![command.script.replace(broken_cmd, correct_cmd)])
 