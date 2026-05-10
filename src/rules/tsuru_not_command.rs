use super::{Command, Rule};
use regex::Regex;

pub struct TsuruNotCommand;

impl Rule for TsuruNotCommand {
    fn name(&self) -> &'static str {
        "tsuru_not_command"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains(" is not a tsuru command. See \"tsuru help\".")
            && cmd.output.contains("\nDid you mean?\n\t")
    }

    fn fix(&self, cmd: &Command) -> String {
        let re = Regex::new(r#"tsuru: "([^"]*)" is not a tsuru command"#).unwrap();
        let broken_cmd = re.captures(&cmd.output)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        let re_suggestions = Regex::new(r"\n\t([^\n]+)").unwrap();
        let suggestions: Vec<&str> = re_suggestions.captures_iter(&cmd.output)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect();

        if suggestions.is_empty() {
            return cmd.text.clone();
        }

        // Replace broken command with the first suggestion
        cmd.text.replace(broken_cmd, suggestions[0])
    }
}