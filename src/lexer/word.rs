use crate::lexer::token::Token;

use super::tag::Tag;

#[derive(Debug, Clone)]
pub struct Word {
    pub lexeme: String,
    pub token: Token,
}

impl Word {
    pub fn new(s: String, tag: Tag) -> Word {
        Word {
            lexeme: s,
            token: Token::new(tag),
        }
    }
}
