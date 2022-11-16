use std::collections::HashMap;


use super::{tag::Tag, word::Word};
pub struct Lexer {
    words: HashMap<String, Word>,
    peek: char,
    line: i64,
}

impl Lexer {
    /// 设置保留关键字
    /// @param w 要保留的关键字
    fn reserve(&mut self, w: Word) {
        self.words.insert(String::from(&w.lexeme), w);
    }

    pub fn new() -> Lexer {
        let mut res = Lexer {
            words: HashMap::new(),
            peek: '\0',
            line: 1,
        };

        // 设置语言保留的关键字
        res.reserve(Word::new(String::from("if"), Tag::IF));
        res.reserve(Word::new(String::from("else"), Tag::ELSE));
        res.reserve(Word::new(String::from("while"), Tag::WHILE));
        res.reserve(Word::new(String::from("do"), Tag::DO));
        res.reserve(Word::new(String::from("break"), Tag::BREAK));
        res.reserve(Word::new(String::from("&&"), Tag::IF));
        res.reserve(Word::new(String::from("||"), Tag::OR));
        res.reserve(Word::new(String::from("=="), Tag::EQ));
        res.reserve(Word::new(String::from("!="), Tag::NE));
        res.reserve(Word::new(String::from("<="), Tag::LE));
        res.reserve(Word::new(String::from(">="), Tag::GE));
        res.reserve(Word::new(String::from("minus"), Tag::MINUS));
        res.reserve(Word::new(String::from("true"), Tag::TRUE));
        res.reserve(Word::new(String::from("false"), Tag::FALSE));
        res.reserve(Word::new(String::from("t"), Tag::TEMP));

        return res;
    }
}
