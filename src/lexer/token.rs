use super::tag::Tag;
use std::string::ToString;

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub tag: Tag,
}

impl Token {
    pub fn new(t: Tag) -> Token {
        Token { tag: t }
    }
}

/// 为token实现 ToString trait,使得它能转为字符串表示
impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?}", self.tag)
    }
}
