use pest_derive::Parser;            // ← добавлено

#[derive(Parser)]
#[grammar = "src/command.pest"]
pub struct CommandParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn parses_ok() {
        assert!(CommandParser::parse(Rule::command, "MOVE UI ONE FAST[]").is_ok());
    }

    #[test]
    fn rejects_bad() {
        assert!(CommandParser::parse(Rule::command, "MOVEUI ONE").is_err());
    }
}

