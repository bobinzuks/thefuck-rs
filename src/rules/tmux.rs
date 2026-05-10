use super::{Command, Rule};
use regex::Regex;

pub struct Tmux;

impl Rule for Tmux {
    fn name(&self) -> &'static str {
        "tmux"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("ambiguous command:")
            && cmd.output.contains("could be:")
    }

    fn fix(&self, cmd: &Command) -> Option<String> {
        let re = Regex::new(r"ambiguous command: (.*), could be: (.*)").ok()?;
        let caps = re.captures(&cmd.output)?;
        
        let old_cmd = caps.get(1)?.as_str().to_string();
        let suggestions: Vec<&str> = caps.get(2)?.as_str()
            .split(',')
            .map(|s| s.trim())
            .collect();
        
        // Find the best match from suggestions
        suggestions.iter()
            .find(|&&suggestion| cmd.text.contains(suggestion) || suggestion.contains(&cmd.text))
            .map(|&suggestion| suggestion.to_string())
            .or_else(|| {
                // Fallback: return the first suggestion if text contains old_cmd
                if cmd.text.contains(&old_cmd) {
                    suggestions.first().map(|&s| s.to_string())
                } else {
                    None
                }
            })
    }
}