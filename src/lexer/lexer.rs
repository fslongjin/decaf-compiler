use std::{collections::HashMap, fs::File, io::Read, process::exit};

use super::{
    annotation::Annotation,
    number::{Number, Real},
    tag::Tag,
    token::Token,
};
pub struct Lexer {
    words: HashMap<String, Box<Token>>,
    peek: char,
    line: i64,
    content: String,
    content_index: usize,
    annotation_state: Annotation, // 当前扫描位置是否位于注释中
    invalid_syms: Vec<char>,      // 不可用字符
}

impl Lexer {
    /// 设置保留关键字
    /// @param w 要保留的关键字
    fn reserve(&mut self, w: Box<Token>) {
        self.words.insert(w.as_ref().lexme.clone().unwrap(), w);
    }

    pub fn new(f: &mut File) -> Lexer {
        let mut res = Lexer {
            words: HashMap::new(),
            peek: ' ',
            line: 1,
            content: String::from(""),
            content_index: 0,
            annotation_state: Annotation::NULL,
            invalid_syms: Vec::from(['@']),
        };

        if f.read_to_string(&mut res.content).is_err() {
            panic!("Lexer: Error occurred while reading input file.");
        }

        // 设置语言保留的关键字
        res.reserve(Token::new_word(String::from("if"), Tag::IF));
        res.reserve(Token::new_word(String::from("else"), Tag::ELSE));
        res.reserve(Token::new_word(String::from("while"), Tag::WHILE));
        res.reserve(Token::new_word(String::from("for"), Tag::FOR));
        res.reserve(Token::new_word(String::from("do"), Tag::DO));
        res.reserve(Token::new_word(String::from("break"), Tag::BREAK));
        res.reserve(Token::new_word(String::from("&&"), Tag::AND));
        res.reserve(Token::new_word(String::from("||"), Tag::OR));
        res.reserve(Token::new_word(String::from("=="), Tag::EQ));
        res.reserve(Token::new_word(String::from("!="), Tag::NE));
        res.reserve(Token::new_word(String::from("<="), Tag::LE));
        res.reserve(Token::new_word(String::from(">="), Tag::GE));
        res.reserve(Token::new_word(String::from("minus"), Tag::MINUS));
        res.reserve(Token::new_word(String::from("true"), Tag::TRUE));
        res.reserve(Token::new_word(String::from("false"), Tag::FALSE));
        res.reserve(Token::new_word(String::from("t"), Tag::TEMP));

        return res;
    }

    /// @brief 获取文件中的下一个字符，如果文件已经读取完毕，则将peek设置为\0
    pub fn get_next_char(&mut self) {
        if self.content_index < self.content.chars().count() {
            self.peek = self.content.chars().nth(self.content_index).unwrap();
            self.content_index += 1;
        } else {
            self.peek = '\0';
        }
    }
    /// @brief 读取下一个字符，如果下一个字符与c相匹配，则返回True，并消费掉peek的内容，否则返回false
    pub fn read_char(&mut self, c: char) -> bool {
        self.get_next_char();
        if self.peek != c {
            return false;
        }
        self.peek = ' ';
        return true;
    }

    fn __match_dup_scan(&mut self, c: char) -> Box<Token> {
        if self.read_char(c) {
            return self.words[(String::from(c) + String::from(c).as_str()).as_str()].clone();
        } else {
            return Token::new_operator(c as i32);
        }
    }
    /// @brief 扫描输入，并返回1个token
    pub fn scan(&mut self) -> Box<Token> {
        loop {
            loop {
                if self.peek == ' ' || self.peek == '\t' {
                    self.get_next_char();
                    continue;
                } else if self.peek == '\n' {
                    self.new_line();
                } else if self.in_annotation() {
                    if self.peek == '*' && self.annotation_state == Annotation::STAR {
                        // 判断能否退出注释
                        if self.read_char('/') {
                            self.annotation_state = Annotation::NULL;
                        }
                    }
                    self.get_next_char();
                    continue;
                } else {
                    break;
                }
                self.get_next_char();
            }

            // 遇到不可用字符，退出
            if !self.is_valid(self.peek) {
                println!(
                    "Found an invalid character “{}” at: Line:{}",
                    self.peek, self.line
                );
                exit(-1);
            }

            match self.peek {
                '&' | '|' | '=' => {
                    return self.__match_dup_scan(self.peek);
                }
                '!' => {
                    if self.read_char('-') {
                        return self.words["!="].clone();
                    } else {
                        return Token::new_operator('!' as i32);
                    }
                }
                '<' => {
                    if self.read_char('=') {
                        return self.words["<="].clone();
                    } else {
                        return Token::new_operator('<' as i32);
                    }
                }
                '>' => {
                    if self.read_char('=') {
                        return self.words[">="].clone();
                    } else {
                        return Token::new_operator('>' as i32);
                    }
                }
                '/' => {
                    self.get_next_char();
                    let nxt = self.peek;
                    match nxt {
                        '/' => {
                            self.annotation_state = Annotation::SLASH;
                            self.peek = ' ';
                            continue;
                        }
                        '*' => {
                            self.annotation_state = Annotation::STAR;
                            self.peek = ' ';
                            continue;
                        }
                        _ => {
                            return Token::new_operator('/' as i32);
                        }
                    }
                }
                _ => {}
            }
            // 将数字转换为token
            if self.peek.is_ascii_digit() {
                let mut value: i32 = 0;
                loop {
                    value = 10 * value + self.peek.to_digit(10).unwrap() as i32;
                    self.get_next_char();

                    if !self.peek.is_ascii_digit() {
                        break;
                    }
                }
                // 获得一个整数
                if self.peek != '.' {
                    return Token::new_number(Number { value });
                }

                let mut x: f64 = value as f64;
                let mut d = 10.0;
                loop {
                    self.get_next_char();
                    if !self.peek.is_ascii_digit() {
                        break;
                    }

                    x = x + (self.peek.to_digit(10).unwrap() as f64) / d;
                    d *= 10.0;
                }

                return Token::new_real(Real { value: x });
            }

            // 普通的identifier
            if self.peek.is_ascii_alphabetic() {
                let mut s = String::new();
                loop {
                    s.push(self.peek);
                    self.get_next_char();
                    if !(self.peek.is_ascii_alphanumeric()) {
                        break;
                    }
                }
                let w = self.words.get(&s);
                if w.is_some() {
                    return w.unwrap().clone();
                } else {
                    let w = Token::new_word(s.clone(), Tag::ID);
                    self.words.insert(s.clone(), w);
                }
                return self.words.get(&s).unwrap().clone();
            }

            // 其他的operator
            if self.peek != '\0' {
                let tok = Token::new_operator(self.peek as i32);
                self.peek = ' ';
                return tok;
            } else {
                return Token::new_eof();
            }
        }
    }

    /// @brief 处理文件换行
    fn new_line(&mut self) {
        self.line += 1;
        // 如果当前位于以 //开头的换行符中，那么换行将会退出注释块
        if self.annotation_state == Annotation::SLASH {
            self.annotation_state = Annotation::NULL;
        }
    }

    /// @brief 判断当前是否位于注释块中
    #[inline]
    fn in_annotation(&self) -> bool {
        if self.annotation_state == Annotation::NULL {
            return false;
        } else {
            return true;
        }
    }

    /// @brief 判断是否在不可用字符列表中
    fn is_valid(&self, c: char) -> bool {
        for x in self.invalid_syms.iter() {
            if *x == c {
                return false;
            }
        }
        return true;
    }

    pub fn get_peek(&self) -> char {
        return self.peek;
    }

    pub fn generate_token(&mut self) -> Vec<Box<Token>> {
        let mut ret: Vec<Box<Token>> = Vec::new();
        loop {
            let token = self.scan();
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
                                println!("(ID , {})", token.lexme.as_ref().unwrap());
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
                                println!("(KEY , {})", token.lexme.as_ref().unwrap());
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
                None => {
                    panic!();
                }
            }
            ret.push(token);
            if self.get_peek() == '\0' {
                break;
            }
        }
        return ret;
    }
}
