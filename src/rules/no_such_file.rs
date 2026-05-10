use super::{Command, Rule};

pub struct NoSuchFile;

impl Rule for NoSuchFile {
    fn name(&self) -> &'static str {
        "no_such_file"
    }

    fn matches(&self, cmd: &Command) -> bool {
        let patterns = [
            r"mv: cannot move '[^']*' to '([^']*)': No such file or directory",
            r"mv: cannot move '[^']*' to '([^']*)': Not a directory",
            r"cp: cannot create regular file '([^']*)': No such file or directory",
            r"cp: cannot create regular file '([^']*)': Not a directory",
        ];

        let re = regex::Regex::new(&patterns.join("|")).unwrap();
        re.is_match(cmd.output.as_str())
    }

    fn fix(&self, cmd: &Command) -> String {
        let patterns = [
            r"mv: cannot move '[^']*' to '([^']*)': No such file or directory",
            r"mv: cannot move '[^']*' to '([^']*)': Not a directory",
            r"cp: cannot create regular file '([^']*)': No such file or directory",
            r"cp: cannot create regular file '([^']*)': Not a directory",
        ];

        let re = regex::Regex::new(&patterns.join("|")).unwrap();
        if let Some(captures) = re.captures(cmd.output.as_str()) {
            let file = captures.get(1).unwrap().as_str();
            if let Some(pos) = file.rfind('/') {
                let dir = &file[..pos];
                return format!("mkdir -p {} && {}", dir, cmd.script);
            }
        }
        cmd.script.clone()
    }
}