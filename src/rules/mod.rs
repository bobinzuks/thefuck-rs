mod git_branch;

pub struct Command {
    pub text: String,
    pub output: String,
    pub exit_code: i32,
}

pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn matches(&self, cmd: &Command) -> bool;
    fn fix(&self, cmd: &Command) -> String;
    fn priority(&self) -> u8 { 50 }
}

pub fn get_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(git_branch::GitBranch),
    ]
}
