Here's a Rust implementation of a `git_add` rule that mirrors common `.gitignore`-like pattern matching logic:

```rust
use std::path::Path;

#[derive(Debug)]
pub struct GitAddRule {
    patterns: Vec<Pattern>,
}

#[derive(Debug)]
enum Pattern {
    // Standard wildcard pattern (e.g., *.log, folder/*)
    Wildcard { 
        has_wildcard: bool,
        prefix: String,
        suffix: String,
    },
    // Directory-specific pattern (ends with /)
    Directory { pattern: String },
    // Exact match
    Exact { value: String },
}

impl GitAddRule {
    pub fn new() -> Self {
        Self { patterns: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: &str) {
        let rule = rule.trim();
        if rule.is_empty() || rule.starts_with('#') {
            return; // Skip comments and empty lines
        }
        
        let pattern = if rule.ends_with('/') {
            Pattern::Directory { 
                pattern: rule.trim_end_matches('/').to_string() 
            }
        } else if rule.contains('*') || rule.contains('?') {
            // Handle wildcards
            let parts: Vec<&str> = rule.split('*').collect();
            Pattern::Wildcard {
                has_wildcard: rule.contains('*'),
                prefix: parts.first().unwrap_or(&"").to_string(),
                suffix: parts.last().unwrap_or(&"").to_string(),
            }
        } else {
            Pattern::Exact { value: rule.to_string() }
        };
        
        self.patterns.push(pattern);
    }

    pub fn matches(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy().replace('\\', "/");
        
        for pattern in &self.patterns {
            match pattern {
                Pattern::Wildcard { has_wildcard: _, prefix, suffix } => {
                    // Simple wildcard matching: prefix*suffix
                    if path_str.starts_with(prefix) && path_str.ends_with(suffix) {
                        if prefix.is_empty() || suffix.is_empty() {
                            return true;
                        }
                        // Ensure the wildcard matches something in the middle
                        let middle = &path_str[prefix.len()..path_str.len() - suffix.len()];
                        if !middle.is_empty() || (prefix.is_empty() && suffix.is_empty()) {
                            return true;
                        }
                    }
                }
                Pattern::Directory { pattern } => {
                    // Check if path is a directory and matches
                    if path.is_dir() {
                        if let Some(dir_name) = path.file_name() {
                            if dir_name == pattern.as_str() {
                                return true;
                            }
                        }
                    }
                }
                Pattern::Exact { value } => {
                    if path_str == *value || path_str.ends_with(&format!("/{}", value)) {
                        return true;
                    }
                }
            }
        }
        
        false
    }

    pub fn should_add(&self, path: &Path) -> bool {
        // If no rules defined, allow all files
        if self.patterns.is_empty() {
            return true;
        }
        
        self.matches(path)
    }

    pub fn load_rules(&mut self, rules: &[&str]) {
        for rule in rules {
            self.add_rule(rule);
        }
    }
}

// Example usage
fn main() {
    let mut rules = GitAddRule::new();
    
    // Add some example rules (similar to git add patterns)
    rules.load_rules(&[
        "*.log",
        "*.tmp",
        "node_modules/",
        "dist/",
        "config.json",
    ]);
    
    let test_paths = vec![
        "file.log",
        "src/main.rs",
        "config.json",
        "node_modules/package/index.js",
        "dist/bundle.js",
    ];
    
    println!("Testing git add rules for files:");
    for path_str in test_paths {
        let path = Path::new(path_str);
        let should_add = !rules.should_add(&path); // Invert because these are exclusion rules
        println!("  {} -> {}", path_str, if should_add { "ADD" } else { "IGNORE" });
    }
    
    // Alternative: Using as inclusion rules (direct git add patterns)
    let mut add_rules = GitAddRule::new();
    add_rules.load_rules(&[
        "src/**/*.rs",
        "*.toml",
        "README.md",
    ]);
    
    println!("\nTesting git add inclusion rules:");
    let test_files = vec!["src/main.rs", "src/lib.rs", "Cargo.toml", "README.md", "node_modules/package.json"];
    for file in test_files {
        let path = Path::new(file);
        let should_add = add_rules.should_add(&path);
        println!("  {} -> {}", file, if should_add { "ADD" } else { "IGNORE" });
    }
}
```

Here's also a more comprehensive version with extended features:

```rust
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
pub struct GitAddRule {
    patterns: Vec<CompiledPattern>,
}

#[derive(Debug)]
enum CompiledPattern {
    // Simple glob pattern
    Glob {
        pattern: String,
        regex: Option<Regex>,
        is_negative: bool,
    },
    // Directory pattern
    Directory {
        dir_name: String,
        is_negative: bool,
    },
    // Exact match
    Exact {
        value: String,
        is_negative: bool,
    },
}

impl GitAddRule {
    pub fn new() -> Self {
        Self { patterns: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: &str) {
        let rule = rule.trim();
        if rule.is_empty() || rule.starts_with('#') {
            return;
        }

        let (is_negative, pattern) = if rule.starts_with('!') {
            (true, &rule[1..])
        } else {
            (false, rule)
        };

        let pattern = pattern.trim();
        if pattern.is_empty() {
            return;
        }

        let compiled = if pattern.ends_with('/') {
            CompiledPattern::Directory {
                dir_name: pattern.trim_end_matches('/').to_string(),
                is_negative,
            }
        } else if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
            // Convert glob to regex (simplified version)
            let regex_pattern = glob_to_regex(pattern);
            CompiledPattern::Glob {
                pattern: pattern.to_string(),
                regex: Regex::new(&regex_pattern).ok(),
                is_negative,
            }
        } else {
            CompiledPattern::Exact {
                value: pattern.to_string(),
                is_negative,
            }
        };

        self.patterns.push(compiled);
    }

    pub fn matches(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy().replace('\\', "/");
        
        for pattern in &self.patterns {
            let matched = match pattern {
                CompiledPattern::Glob { regex, pattern: _, is_negative: _ } => {
                    if let Some(re) = regex {
                        re.is_match(&path_str) || 
                        re.is_match(path.file_name().unwrap_or_default().to_str().unwrap_or(""))
                    } else {
                        false
                    }
                }
                CompiledPattern::Directory { dir_name, is_negative: _ } => {
                    path_str == *dir_name || 
                    path_str.starts_with(&format!("{}/", dir_name)) ||
                    path_str.ends_with(&format!("/{}", dir_name)) ||
                    path_str.contains(&format!("/{}/", dir_name))
                }
                CompiledPattern::Exact { value, is_negative: _ } => {
                    path_str == *value || 
                    path_str.ends_with(&format!("/{}", value)) ||
                    path.file_name().map_or(false, |n| n == value.as_str())
                }
            };

            if matched {
                return !pattern.is_negative();
            }
        }
        
        false
    }

    pub fn should_include(&self, path: &Path) -> bool {
        self.matches(path)
    }

    pub fn should_exclude(&self, path: &Path) -> bool {
        !self.matches(path)
    }

    pub fn load_rules(&mut self, rules: &[&str]) {
        for rule in rules {
            self.add_rule(rule);
        }
    }
}

fn glob_to_regex(glob: &str) -> String {
    let mut regex = String::new();
    regex.push('^');
    
    let mut chars = glob.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '*' => {
                if chars.peek() == Some(&'*') {
                    chars.next(); // consume second *
                    if chars.peek() == Some(&'/') {
                        chars.next(); // consume /
                        regex.push_str("(.*/)?");
                    } else {
                        regex.push_str(".*");
                    }
                } else {
                    regex.push_str("[^/]*");
                }
            }
            '?' => regex.push_str("[^/]"),
            '.' | '+' | '^' | '$' | '(' | ')' | '|' | '{' | '}'