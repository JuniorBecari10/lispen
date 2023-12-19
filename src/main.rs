mod util;
mod token;
mod lexer;

fn main() {
    repl();
}

fn repl() -> Option<()> {
    println!("Lispen REPL\n");

    loop {
        let mut input = String::new();
        util::input("> ", &mut input);

        if input == "exit" {
            return Some(());
        }

        process_input(input)?;
    }
}

fn process_input(input: String) -> Option<()> {
    let lexer_res = lexer::Lexer::new(&input).lex();
    if lexer_res.1 { return None; }

    let tokens = lexer_res.0;

    for t in tokens {
        println!("{:?}", t);
    }

    Some(())
}
