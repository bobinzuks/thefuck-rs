use super::{Command, Rule};

pub struct Open;

impl Rule for Open {
    fn name(&self) -> &'static str {
        "open"
    }

    fn matches(&self, cmd: &Command) -> bool {
        let is_url = cmd.text.contains(".com")
            || cmd.text.contains(".edu")
            || cmd.text.contains(".info")
            || cmd.text.contains(".io")
            || cmd.text.contains(".ly")
            || cmd.text.contains(".me")
            || cmd.text.contains(".net")
            || cmd.text.contains(".org")
            || cmd.text.contains(".se")
            || cmd.text.contains("www.");

        is_url
            || (cmd.output.trim_start().starts_with("The file ")
                && cmd.output.trim_end().ends_with(" does not exist."))
    }

    fn fix(&self, cmd: &Command) -> String {
        let output = cmd.output.trim();
        let is_url = cmd.text.contains(".com")
            || cmd.text.contains(".edu")
            || cmd.text.contains(".info")
            || cmd.text.contains(".io")
            || cmd.text.contains(".ly")
            || cmd.text.contains(".me")
            || cmd.text.contains(".net")
            || cmd.text.contains(".org")
            || cmd.text.contains(".se")
            || cmd.text.contains("www.");

        if is_url {
            cmd.text.replacen("open ", "open http://", 1)
        } else if output.starts_with("The file ") && output.ends_with(" does not exist.") {
            let parts: Vec<&str> = cmd.text.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let arg = parts[1];
                format!("open http://{}", arg)
            } else {
                cmd.text.clone()
            }
        } else {
            cmd.text.clone()
        }
    }
}