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

        let lexer_res = lexer::Lexer::new(&input).lex();
        if lexer_res.is_err() { continue; }

        let parser_res = parser::Parser::new(lexer_res.unwrap()).parse();
        if parser_res.is_err() { continue; }

        interpreter::interpret(parser_res.unwrap(), &mut env);
    }
}
