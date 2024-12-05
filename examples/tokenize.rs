use lexer_and_parser::Lexer;

fn main() {
    let source_code = std::fs::read_to_string("tokenize.c").expect("Failed to read source file");
    
    let tokens = Lexer::new(&source_code).parse();
    for line in tokens.iter() {
        for token in &line.tokens {
            println!("{:02}:{:02} {}, '{}'",
                     token.row,
                     token.column,
                     token.token.kind(),
                     token.token.raw());
        }
    }
}
