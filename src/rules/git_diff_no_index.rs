pub struct GitDiffNoIndex;

impl Rule for GitDiffNoIndex {
    fn name(&self) -> &'static str {
        "git_diff_no_index"
    }

    fn matches(&self, cmd: &Command) -> bool {
        let parts: Vec<&str> = cmd.text.split_whitespace().collect();
        let files: Vec<&&str> = parts[2..].iter()
            .filter(|arg| !arg.starts_with('-'))
            .collect();
        
        cmd.text.contains("diff") 
            && !cmd.text.contains("--no-index")
            && files.len() == 2
    }

    fn fix(&self, cmd: &Command) -> String {
        cmd.text.replacen("diff", "diff --no-index", 1)
    }
}