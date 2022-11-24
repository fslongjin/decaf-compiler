/// @brief 当前词法分析器正在扫描的位置所处的注释类型
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Annotation{
    NULL,   /// 当前不位于注释中
    STAR,   /// 当前位于以*号开头的注释中
    /// 当前位于以 // 开头的注释中
    SLASH,   
}

impl PartialEq for Annotation{
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}