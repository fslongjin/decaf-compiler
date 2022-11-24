use std::fs::File;

use crate::lexer::tag::Tag;

pub mod lexer;
mod utils;

fn main() {
    let mut f = File::open("example/example1.decaf").unwrap();
    let mut lex = lexer::lexer::Lexer::new(&mut f);
    let mut tokens = lex.generate_token();
}
