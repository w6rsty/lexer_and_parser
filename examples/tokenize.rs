use lexer_and_parser::{Lexer, Token};

fn main() {
    let source_code = std::fs::read_to_string("test_code/tokenize_error.c").expect("Failed to read source file");
    
    let token_lines = Lexer::new(&source_code).parse();
    // for line in token_lines.iter().filter(|line| {
    //     !line.tokens.is_empty() && line.tokens.iter().any(|t| !matches!(t.token, Token::Comment(_)))
    // }) {
    //     print!("{:2}\t", line.line_number);
    //     line.tokens.iter()
    //         .filter(|t| !matches!(t.token, Token::Comment(_)))
    //         .for_each(|token| print!("{} ", token.token.raw()));
    //     println!();
    // }

    for line in token_lines.iter() {
        line.tokens
            .iter()
            .filter(|t| !matches!(t.token, Token::Comment(_)))
            .for_each(|token| {
                println!("{:?} ", token);
            });
    }
}
