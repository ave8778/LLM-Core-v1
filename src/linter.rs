use regex::Regex;

#[derive(Debug)]
pub enum RuleId { DuplicateAction, ForbiddenWords }

#[derive(Debug)]
pub struct LintIssue { pub rule: RuleId, pub msg: String }

pub fn lint(src: &str) -> Vec<LintIssue> {
    let mut issues = Vec::new();
    let dup = Regex::new(r"(MOVE.+){2,}").unwrap();
    if dup.is_match(src) {
        issues.push(LintIssue { rule: RuleId::DuplicateAction,
            msg: "Combine repeated MOVE segments".into() });
    }
    let forb = Regex::new(r"(SLOW|UNSAFE)").unwrap();
    if forb.is_match(src) {
        issues.push(LintIssue { rule: RuleId::ForbiddenWords,
            msg: "Use valid modifiers: FAST, SAFE, RECURSE".into() });
    }
    issues
}
