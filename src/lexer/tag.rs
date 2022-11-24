#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Tag {
    AND = 256,
    BASIC,
    BREAK,
    DO,
    ELSE,
    EQ,
    FALSE,
    GE,
    ID,
    IF,
    INDEX,
    LE,
    MINUS,
    NE,
    NUM,
    OR,
    REAL,
    TEMP,
    TRUE,
    WHILE,
    FOR,
    EOF,
    __MAX__,
}

impl Tag {
    /// @brief 从i32转换为Tag枚举类型，如果传入的x不符合要求，则返回None
    #[allow(dead_code)]
    pub fn from_i32(x: i32) -> Option<Tag> {
        if Self::valid_tag_number(x) {
            let ret: Tag = unsafe { core::mem::transmute(x) };
            return Some(ret);
        }

        return None;
    }

    /// 判断一个数字是否可用
    fn valid_tag_number(x: i32) -> bool {
        if x > (Tag::AND as i32) && x < (Tag::__MAX__ as i32) {
            return true;
        } else {
            return false;
        }
    }
}