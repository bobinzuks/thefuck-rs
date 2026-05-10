use super::{Command, Rule};
use regex::Regex;

pub struct RailsMigrationsPending;

impl Rule for RailsMigrationsPending {
    fn name() -> &'static str {
        "rails_migrations_pending"
    }

    fn matches(cmd: &Command) -> bool {
        cmd.output.contains("Migrations are pending. To resolve this issue, run:")
    }

    fn fix(cmd: &Command) -> String {
        let suggestion_regex = Regex::new(r"To resolve this issue, run:\s+(.*?)\n").unwrap();
        let migration_script = suggestion_regex
            .captures(&cmd.output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");
        
        format!("{} && {}", migration_script, cmd.script)
    }
}