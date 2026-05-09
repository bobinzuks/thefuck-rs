use super::{Command, Rule};

pub struct Cpp11;

impl Rule for Cpp11 {
    fn name(&self) -> &'static str {
        "cpp11"
    }

    fn is_match(&self, command: &Command) -> bool {
        command.output.contains("This file requires compiler and library support for the ISO C++ 2011 standard.")
            || command.output.contains("-Wc++11-extensions")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec![format!("{} -std=c++11", command.script)]
    }

    fn is_applicable(&self, command: &Command) -> bool {
        command.script.starts_with("g++") || command.script.starts_with("clang++")
    }
}
