use lexer_and_parser::{Lexer, Parser};

fn main() {
    let correct = std::fs::read_to_string("parse_correct.c").expect("Failed to read source file");
    let incorrect = std::fs::read_to_string("parse_incorrect.c").expect("Failed to read source file");

    println!("===== Parsing correct source code =====");

    let lexer = Lexer::new(&correct);
    let mut parser = Parser::new(lexer);

    if let Some(err) = parser.parse_program().err() {
        eprintln!("{}", err);
    } else {
        println!("Parsed successfully");
    }

    println!("\n=====Parsing incorrect source code =====");

    let lexer = Lexer::new(&incorrect);
    let mut parser = Parser::new(lexer);

    if let Some(err) = parser.parse_program().err() {
        eprintln!("{}", err);
    } else {
        println!("Parsed successfully");
    }
}
