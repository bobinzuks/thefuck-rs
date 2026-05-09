mod git_branch;
mod git_push;
mod sudo;
mod cd_mkdir;
mod python_command;

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
        Box::new(sudo::Sudo),
        Box::new(git_branch::GitBranch),
        Box::new(git_push::GitPush),
        Box::new(cd_mkdir::CdMkdir),
        Box::new(python_command::PythonCommand),
    ]
}
