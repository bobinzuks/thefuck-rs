use super::{Command, Rule};

pub struct PhpS;

impl Rule for PhpS {
    fn name(&self) -> &'static str {
        "php_s"
    }

    fn matches(&self, command: &Command) -> bool {
        command.text.contains("php ")
            && command.text.split_whitespace().skip(1).any(|part| part == "-s")
            && command.text.split_whitespace().last().map_or(false, |last| last != "-s")
    }

    fn fix(&self, command: &Command) -> String {
        let parts: Vec<&str> = command.text.splitn(3, ' ').collect();
        if parts.len() >= 3 {
            format!("{} {} {}", parts[0], "-S", command.text.splitn(3, ' ').skip(2).collect::<Vec<&str>>().join(" "))
        } else {
            command.text.replace("-s", "-S")
        }
    }
}