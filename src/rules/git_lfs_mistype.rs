use super::{Command, Rule};
use regex::Regex;

pub struct GitLfsMistype;

impl Rule for GitLfsMistype {
    fn name(&self) -> &'static str {
        "git_lfs_mistype"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("lfs") && cmd.output.contains("Did you mean this?")
    }

    fn fix(&self, cmd: &Command) -> String {
        let re = Regex::new(r#"Error: unknown command "([^"]*)" for "git-lfs""#).unwrap();
        let broken_cmd = re.captures(&cmd.output)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        if broken_cmd.is_empty() {
            return cmd.text.clone();
        }

        // Parse suggestions from output
        let suggestions: Vec<String> = cmd.output
            .lines()
            .filter(|line| line.contains("Did you mean") || line.contains(" for usage."))
            .filter_map(|line| {
                // Extract the suggested command from lines like "Did you mean: lfs_pull?"
                if let Some(start) = line.find(':') {
                    let suggestion = line[start+1..].trim().trim_matches('.').trim().to_string();
                    if !suggestion.is_empty() {
                        return Some(suggestion);
                    }
                }
                None
            })
            .collect();

        if suggestions.is_empty() {
            return cmd.text.clone();
        }

        // Replace the broken command with the first suggestion
        cmd.text.replace(broken_cmd, &suggestions[0])
    }
}