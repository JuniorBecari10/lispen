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
    let mut env = env::Environment::new();

    loop {
        let mut input = String::new();
        util::input("> ", &mut input);

        if input == "exit" {
            return;
        }

        process_input(input, &mut env);
    }
}

fn process_input(input: String, env: &mut env::Environment) {
    let lexer_res = lexer::Lexer::new(&input).lex();
    if lexer_res.1 { return; }

    let parser_res = parser::Parser::new(lexer_res.0).parse();
    if parser_res.1 { return; }

    interpreter::Interpreter::new(parser_res.0, env).interpret();
}
