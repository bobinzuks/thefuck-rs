use super::{Command, Rule};

pub struct GitRebaseMergeDir;

impl Rule for GitRebaseMergeDir {
    fn name(&self) -> &'static str {
        "git_rebase_merge_dir"
    }

    fn matches(&self, command: &Command) -> bool {
        command.text.contains(" rebase")
            && command.output.contains("It seems that there is already a rebase-merge directory")
            && command.output.contains("I wonder if you are in the middle of another rebase")
    }

    fn fix(&self, command: &Command) -> String {
        let command_list = vec![
            "git rebase --continue".to_string(),
            "git rebase --abort".to_string(),
            "git rebase --skip".to_string(),
        ];
        
        let lines: Vec<&str> = command.output.split('\n').collect();
        if lines.len() >= 4 {
            let rm_cmd = lines[lines.len() - 4].trim().to_string();
            command_list.push(rm_cmd);
        }
        
        get_close_matches(&command.text, &command_list, 4, 0)
    }
}

fn get_close_matches(text: &str, candidates: &[String], n: usize, cutoff: f64) -> String {
    candidates
        .iter()
        .min_by_key(|candidate| levenshtein_distance(text, candidate))
        .cloned()
        .unwrap_or_else(|| text.to_string())
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
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost,
            );
        }
    }

    matrix[a_len][b_len]
}