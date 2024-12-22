use lexer_and_parser::{Lexer, Parser};

fn main() {
    let correct = std::fs::read_to_string("test_code/parse_correct.c").expect("Failed to read source file");
    let incorrect = std::fs::read_to_string("test_code/parse_incorrect.c").expect("Failed to read source file");

    println!("===== Parsing correct source code =====");

    for token_line in Lexer::new(&correct).parse() {
        for token in token_line.tokens.iter() {
            println!("{:02}:{:02} {:?}", token.row, token.column, token.token);
        }
    }

    let mut parser = Parser::new(Lexer::new(&correct));

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
