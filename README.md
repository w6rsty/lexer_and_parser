# Simple C-like lexer and parser

## Run example

Lexer:

```bash
cargo run --example tokenize
```

Parser:
```bash
cargo run --example parse
```

Recursive descent parser

grammer
```
program -> block

block   -> { stmts }

stmts   -> stmts stmts | ε

stmt    -> id = expr;
         | if (bool) stmt
         | if (bool) stmt else stmt
         | while (bool) stmt
         | do stmt while (bool)
         | break
         | block

bool    -> expr < expr
         | expr <= expr
         | expr > expr
         | expr >= expr
         | expr

expr    -> expr + term
         | expr - term
         | term

term    -> term * factor
         | term / factor
         | factor
         
factor  -> (expr) | id | num
```


