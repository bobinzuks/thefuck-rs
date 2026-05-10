pub struct GoUnknownCommand;

impl Rule for GoUnknownCommand {
    fn name(&self) -> &'static str {
        "go_unknown_command"
    }

    fn matches(&self, command: &Command) -> bool {
        command.output.contains("unknown command")
    }

    fn fix(&self, command: &Command) -> String {
        let go_commands = get_golang_commands();
        let closest = get_closest(&command.script_parts[1], &go_commands);
        replace_argument(&command.script, &command.script_parts[1], &closest)
    }
}

fn get_golang_commands() -> Vec<String> {
    use std::process::Command as StdCommand;
    
    let output = StdCommand::new("go")
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|child| child.wait_with_output())
        .ok()
        .and_then(|output| String::from_utf8(output.stderr).ok());
    
    match output {
        Some(stderr) => {
            stderr.lines()
                .map(|line| line.trim())
                .skip_while(|line| *line != "The commands are:")
                .skip(2)
                .take_while(|line| !line.is_empty())
                .map(|line| line.split(' ').next().unwrap_or("").to_string())
                .collect()
        }
        None => Vec::new()
    }
}

fn get_closest(input: &str, candidates: &[String]) -> String {
    // Simple Levenshtein-based closest match
    candidates.iter()
        .min_by_key(|candidate| levenshtein_distance(input, candidate))
        .cloned()
        .unwrap_or_else(|| input.to_string())
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();
    
    let mut matrix = vec![vec![0usize; b_len + 1]; a_len + 1];
    
    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }
    
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i-1] == b_chars[j-1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                matrix[i-1][j] + 1,
                std::cmp::min(
                    matrix[i][j-1] + 1,
                    matrix[i-1][j-1] + cost
                )
            );
        }
    }
    
    matrix[a_len][b_len]
}

fn replace_argument(script: &str, old_arg: &str, new_arg: &str) -> String {
    script.replacen(old_arg, new_arg, 1)
}