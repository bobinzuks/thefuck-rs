pub struct PythonExecute;

impl Rule for PythonExecute {
    fn name(&self) -> &'static str {
        "python_execute"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.starts_with("python ") && !cmd.text.trim_end().ends_with(".py")
    }

    fn fix(&self, cmd: &Command) -> String {
        format!("{}.py", cmd.text.trim_end())
    }
}