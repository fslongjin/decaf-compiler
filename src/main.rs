use std::fs::File;

use crate::lexer::tag::Tag;

pub mod lexer;

fn main() {
    let mut f = File::open("example/example1.decaf").unwrap();
    let mut lex = lexer::lexer::Lexer::new(&mut f);
    loop {
        let token = lex.scan();
        // println!("token={:?}", token);
        match token.tag {
            Some(t) => {
                let tg = Tag::from_i32(t);
                if tg.is_some() {
                    match tg.unwrap() {
                        Tag::NUM => {
                            println!("(NUM , {})", token.number.unwrap().value);
                        }
                        Tag::REAL => {
                            println!("(NUM , {})", token.real.unwrap().value);
                        }
                        Tag::ID => {
                            println!("(ID , {})", token.lexme.unwrap());
                        }
                        Tag::AND
                        | Tag::BASIC
                        | Tag::BREAK
                        | Tag::DO
                        | Tag::ELSE
                        | Tag::IF
                        | Tag::TRUE
                        | Tag::WHILE
                        | Tag::FOR => {
                            println!("(KEY , {})", token.lexme.unwrap());
                        }
                        _ => {}
                    }
                } else if t != 13 {
                    let tk = Tag::from_i32(token.tag.unwrap());
                    if tk.is_some() {
                        println!("(SYM , {:?})", tk);
                    } else {
                        println!("(SYM , {})", token.tag.unwrap() as u8 as char);
                    }
                }
            }
            None => {}
        }
        if lex.get_peek() == '\0' {
            break;
        }
    }
}
