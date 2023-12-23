mod util;
mod token;
mod lexer;
mod expr;
mod parser;
mod env;
mod interpreter;

fn main() {
    repl();
}

fn repl() {
    println!("Lispen REPL\n");

    loop {
        let mut input = String::new();
        util::input("> ", &mut input);

        if input == "exit" {
            return;
        }

        process_input(input);
    }
}

fn process_input(input: String) {
    let lexer_res = lexer::Lexer::new(&input).lex();
    if lexer_res.1 { return; }

    for t in &lexer_res.0 {
        println!("{:?}", t);
    }

    let parser_res = parser::Parser::new(lexer_res.0).parse();
    if parser_res.1 { return; }

    interpreter::Interpreter::new(parser_res.0).interpret();
}
