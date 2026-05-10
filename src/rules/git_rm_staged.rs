pub struct GitRmStaged;

impl Rule for GitRmStaged {
    fn name(&self) -> &'static str {
        "git_rm_staged"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains(" rm ")
            && cmd.output.contains("error: the following file has changes staged in the index")
            && cmd.output.contains("use --cached to keep the file, or -f to force removal")
    }

    fn fix(&self, cmd: &Command) -> String {
        let mut parts: Vec<&str> = cmd.text.split_whitespace().collect();
        let rm_index = parts.iter().position(|&s| s == "rm").unwrap() + 1;
        parts.insert(rm_index, "--cached");
        let first_cmd = parts.join(" ");
        parts[rm_index] = "-f";
        let second_cmd = parts.join(" ");
        format!("{}\n{}", first_cmd, second_cmd)
    }
}