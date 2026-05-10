pub struct GitTwoDashes;

impl Rule for GitTwoDashes {
    fn name(&self) -> &'static str {
        "git_two_dashes"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("error: did you mean `")
            && cmd.output.contains("` (with two dashes ?)")
    }

    fn fix(&self, cmd: &Command) -> String {
        let to: &str = cmd.output.split('`').nth(1).unwrap_or("");
        let old = &to[1..];
        cmd.text.replacen(old, to, 1)
    }
}