use super::{Command, Rule};

pub struct RuleDeleteGhostMigrations;

impl Rule for RuleDeleteGhostMigrations {
    fn match(&self, command: &Command) -> bool {
        command.script.contains("manage.py")
            && command.script.contains("migrate")
            && command.output.contains("or pass --delete-ghost-migrations")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec![format!("{} --delete-ghost-migrations", command.script)]
    }
}
