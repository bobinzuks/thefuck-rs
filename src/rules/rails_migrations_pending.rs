use super::{Command, Rule};

pub struct RailsMigrationsPending;

impl Rule for RailsMigrationsPending {
    fn name(&self) -> &str { "rails_migrations_pending" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("rails") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
