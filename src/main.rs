#[macro_use]
extern crate lazy_static;

mod token;

use std::io::{self, Write};
use token::*;


fn main() {
    repl();
}

fn run(source: &String) -> Vec<Token> {
    let r = build_tokenizer(source);
    println!("");
    io::stdout().flush().unwrap();
    r
}

fn repl() {
    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        print!("cq)");
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    return;
                }
                let output = run(&input);
                print!("{:?}", output);
            }
            Err(err) => println!("Error! {}", err),
        }
        input = String::new();
    }
}
