use super::{Command, Rule};

pub struct TsuruLogin;

impl Rule for TsuruLogin {
    fn name(&self) -> &'static str {
        "tsuru_login"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("not authenticated")
            && cmd.output.contains("session has expired")
    }

    fn fix(&self, cmd: &Command) -> String {
        format!("tsuru login && {}", cmd.text)
    }
}