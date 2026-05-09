use super::{Command, Rule};

pub struct RuleMergeMigration;

impl Rule for RuleMergeMigration {
    fn match(&self, command: &Command) -> bool {
        command.script.contains("manage.py")
            && command.script.contains("migrate")
            && command.output.contains("--merge: will just attempt the migration")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec![format!("{} --merge", command.script)]
    }
}
