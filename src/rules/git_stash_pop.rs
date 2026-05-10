use super::{Command, Rule};

pub struct GitStashPop;

impl Rule for GitStashPop {
    fn name(&self) -> &'static str {
        "git_stash_pop"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("stash")
            && cmd.text.contains("pop")
            && cmd.output.contains("Your local changes to the following files would be overwritten by merge")
    }

    fn fix(&self, cmd: &Command) -> String {
        // Equivalent to shell.and_('git add --update', 'git stash pop', 'git reset .')
        format!("{} && {} && {}",
            "git add --update",
            "git stash pop",
            "git reset ."
        )
    }

    fn priority(&self) -> u32 {
        900
    }
}