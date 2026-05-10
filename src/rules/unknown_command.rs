use super::{Command, Rule};
use regex::Regex;

pub struct UnknownCommand;

impl Rule for UnknownCommand {
    fn name(&self) -> &'static str {
        "unknown_command"
    }

    fn matches(&self, cmd: &Command) -> bool {
        let output_unknown = Regex::new(r"([^:]*): Unknown command.*").unwrap();
        let output_did_you_mean = Regex::new(r"Did you mean ([^?]*)?").unwrap();
        
        output_unknown.is_match(&cmd.output) && output_did_you_mean.is_match(&cmd.output)
    }

    fn fix(&self, cmd: &Command) -> String {
        let broken_re = Regex::new(r"([^:]*): Unknown command.*").unwrap();
        let matched_re = Regex::new(r"Did you mean ([^?]*)?").unwrap();
        
        let broken_cmd = broken_re.captures(&cmd.output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");
            
        let matched: Vec<&str> = matched_re.captures_iter(&cmd.output)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str())
            .collect();
            
        // Generate alternative commands from suggestions
        let cmd_text = &cmd.text;
        let suggestions: Vec<String> = matched.iter()
            .flat_map(|s| s.split(", "))
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|alternative| {
                cmd_text.replace(broken_cmd, alternative)
            })
            .collect();
            
        suggestions.join("\n")
    }
}