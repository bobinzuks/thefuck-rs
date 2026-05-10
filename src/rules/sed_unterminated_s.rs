use super::{Command, Rule};

pub struct SedUnterminatedS;

impl Rule for SedUnterminatedS {
    fn name() -> &'static str {
        "sed_unterminated_s"
    }

    fn matches(command: &Command) -> bool {
        command.output.contains("unterminated `s' command")
    }

    fn fix(command: &Command) -> String {
        let script = command.text.split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();
        let mut script = script.clone();

        for (i, e) in script.iter().enumerate() {
            if (e.starts_with("s/") || e.starts_with("-es/")) && !e.ends_with('/') {
                script[i] = format!("{}/", e);
            }
        }

        script.join(" ")
    }
}