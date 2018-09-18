﻿use std::collections::HashMap;

mod tokens;
mod lexer;
mod parser;
mod expressions;
mod interpreter;

fn main() {
    let mut context = interpreter::InterpreterContext::new();
    context.variable_map.push(HashMap::new());
    context.insert_double(0, "PI".to_owned(), 3.14);

    let input = "
        PI
        2
        0x01F
        \"string\"
        double_var <- 2 + 2 * 2
        string_var <- \"Hello, World!\"
        string_var ><
        double_var <- double_var + (0xAF / 1.5)
        double_var <- null

        print <- function(value) {
            value
        }

        mulBy2 <- function(value) {
            multiplied <- value * 2

            multiplied
        }

        mulBy2(11)

        print(value)
    ";

    let mut lex = lexer::Lexer::new(input);
    lex.tokenize();

    let lex_output = lex.output;

    for x in &lex_output {
        println!("{:?}", x);
    }


    let mut par = parser::Parser::new(&lex_output);
    par.parse();

    let par_output = par.output;

    for x in par_output {
        use expressions::Expression;

        let result = x.eval(&mut context);

        println!("{:?}", x);

        match Box::leak(x) {
            Expression::CallFunc(_key, _args) => println!("{:?}", result),
            _ => { continue; }
        }
    }
}
