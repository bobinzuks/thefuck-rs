use super::{Command, Rule};

pub struct PythonModuleError;

impl Rule for PythonModuleError {
    fn name(&self) -> &'static str {
        "python_module_error"
    }

    fn matches(&self, command: &Command) -> bool {
        command.output.contains("ModuleNotFoundError: No module named '")
    }

    fn fix(&self, command: &Command) -> String {
        let re = regex::Regex::new(r"ModuleNotFoundError: No module named '([^']+)'").unwrap();
        if let Some(captures) = re.captures(&command.output) {
            let missing_module = captures.get(1).map_or("", |m| m.as_str());
            format!("pip install {} && {}", missing_module, command.text)
        } else {
            command.text.clone()
        }
    }
}