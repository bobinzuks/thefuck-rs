pub struct MvnUnknownLifecyclePhase;

impl Rule for MvnUnknownLifecyclePhase {
    fn name(&self) -> &'static str {
        "mvn_unknown_lifecycle_phase"
    }

    fn matches(&self, cmd: &Command) -> bool {
        let failed = get_failed_lifecycle(cmd);
        let available = get_available_lifecycles(cmd);
        available.is_some() && failed.is_some()
    }

    fn fix(&self, cmd: &Command) -> String {
        let failed = get_failed_lifecycle(cmd);
        let available = get_available_lifecycles(cmd);
        
        if let (Some(failed_match), Some(available_match)) = (failed, available) {
            let failed_phase = &failed_match[1];
            let available_phases: Vec<&str> = available_match[1].split(", ").collect();
            
            let closest = get_close_matches(failed_phase, &available_phases);
            if let Some(closest) = closest.first() {
                return cmd.text.replace(failed_phase, closest);
            }
        }
        cmd.text.clone()
    }
}

fn get_failed_lifecycle(cmd: &Command) -> Option<regex::Captures> {
    let re = regex::Regex::new(r#"\[ERROR\] Unknown lifecycle phase "(.+)""#).unwrap();
    re.captures(&cmd.output)
}

fn get_available_lifecycles(cmd: &Command) -> Option<regex::Captures> {
    let re = regex::Regex::new(r"Available lifecycle phases are: (.+) -> \[Help 1\]").unwrap();
    re.captures(&cmd.output)
}

fn get_close_matches<'a>(target: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    use crate::utils::levenshtein;
    
    let mut scores: Vec<(&str, usize)> = candidates
        .iter()
        .map(|&c| (c, levenshtein(target, c)))
        .collect();
    
    scores.sort_by(|a, b| a.1.cmp(&b.1));
    
    let min_score = scores.first().map(|s| s.1).unwrap_or(0);
    scores.into_iter()
        .filter(|s| s.1 == min_score)
        .map(|s| s.0)
        .collect()
}