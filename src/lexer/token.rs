use std::string::ToString;

use super::{number::{Number, Real}, tag::Tag};

#[derive(Debug, Clone)]
pub struct Token {
    pub tag: Option<i32>,
    pub number: Option<Number>,
    pub real: Option<Real>,
    pub lexme: Option<String>,
    
}

impl Token {
    pub fn new_operator(t: i32) -> Box<Token> {
        Box::new(Token {
            tag: Some(t),
            number: None,
            lexme: None,
            real: None,
        })
    }
    pub fn new_number(value: Number) -> Box<Token> {
        Box::new(Token {
            tag: Some(Tag::NUM as i32),
            number: Some(value),
            lexme: None,
            real:None,
        })
    }

    pub fn new_word(s: String, t:Tag) -> Box<Token> {
        Box::new(Token {
            lexme: Some(s),
            tag: Some(t as i32),
            number: None,
            real:None,
        })
    }

    pub fn new_real(v: Real) -> Box<Token>{
        Box::new(Token{
            real: Some(v),
            tag: Some(Tag::REAL as i32),
            number: None,
            lexme: None,
        })
    }
    pub fn new_eof()->Box<Token>{
        Box::new(Token{
            tag: Some(Tag::EOF as i32),
            number: None,
            real: None,
            lexme: None,
        })
    }
}

/// 为token实现 ToString trait,使得它能转为字符串表示
impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?}", self.tag)
    }
}
