use std::collections::HashMap;

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
        2
        0x01F
        \"string\"
        double_var <- 2 + 2 * 2
        string_var <- \"Hello, World!\"
        string_var ><
        double_var <- double_var + (0xAF / 1.5)
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

    for x in &par_output {
        println!("{:?}", x.eval(&mut context));
    }
}
