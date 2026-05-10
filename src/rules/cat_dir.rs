use super::{Command, Rule};

pub struct CatDir;

impl Rule for CatDir {
    fn name(&self) -> &str { "cat_dir" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("cat") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
