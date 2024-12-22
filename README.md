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

stmts   -> stmt stmts | ε

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

```
program -> block

block -> { stmts }

stmts -> stmt stmts | ε

stmt -> id = expr ;
     | if (bool) stmt restIf
     | while (bool) stmt
     | do stmt while (bool)
     | break
     | block

restIf -> else stmt | ε

bool -> expr bop
bop  -> < expr
      | <= expr
      | > expr
      | >= expr
      | ε

expr -> term expr'
expr' -> + term expr'
       | - term expr'
       | ε

term -> factor term'
term' -> * factor term'
       | / factor term'
       | ε

factor -> ( expr ) | id | num
```