use pest_derive::Parser;            // ← добавлено

#[derive(Parser)]
#[grammar = "src/command.pest"]
pub struct CommandParser;
pub mod linter;

#[cfg(test)]
mod lint_tests {
    use super::linter::lint;
    #[test]
    fn catches_duplicate() {
        assert!(!lint("MOVE UI ONE FAST MOVE UI ONE FAST").is_empty());
    }
    #[test]
    fn catches_forbidden() {
        assert!(!lint("MOVE UI ONE SLOW").is_empty());
    }
}


