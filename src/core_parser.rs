//! Core parser for LLM‑Core v1 commands.
//
// This is a placeholder implementation using the Pest parser.

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../bnf.g4"] // path to grammar file
pub struct LLMCoreParser;

fn main() {
    let input = "MOVEUIONEFAST";
    let parse_result = LLMCoreParser::parse(Rule::command, input);
    match parse_result {
        Ok(pairs) => println!("Parsed successfully: {:?}", pairs),
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
