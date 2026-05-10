pub struct LnNoHardLink;

impl Rule for LnNoHardLink {
    fn name(&self) -> &'static str {
        "ln_no_hard_link"
    }

    fn matches(&self, command: &Command) -> bool {
        command.output.ends_with("hard link not allowed for directory")
            && command.text.starts_with("ln ")
    }

    fn fix(&self, command: &Command) -> String {
        command.text.replacen("ln ", "ln -s ", 1)
    }
}