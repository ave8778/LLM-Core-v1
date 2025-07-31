use regex::Regex;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct YRule { id: String, pattern: String, recommendation: String }

#[derive(Debug)]
pub struct LintIssue { pub id: String, pub msg: String }

static RULES: Lazy<Vec<YRule>> = Lazy::new(|| {
    let text = fs::read_to_string(Path::new("lint_rules.yaml"))
        .expect("lint_rules.yaml missing");
    serde_yaml::from_str::<Vec<YRule>>(&text)
        .expect("Invalid YAML in lint_rules.yaml")
});

pub fn lint(src: &str) -> Vec<LintIssue> {
    RULES.iter()
        .filter_map(|r| {
            let re = Regex::new(&r.pattern).ok()?;
            re.is_match(src).then(|| LintIssue {
                id: r.id.clone(),
                msg: r.recommendation.clone(),
            })
        })
        .collect()
}
