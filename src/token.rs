pub enum Token {
    SP_S, // 文字列集合_開始
    SP_E, // 文字列集合_終了
    AND,  // 文字列集合_論理積
    OR,   // 文字列集合_論理和
    INVT, // 文字列集合_否定
    REPT, // 文字列集合_1文字以上の繰返し
    CH_S, // 文字集合_開始
    CH_E, // 文字集合_終了
    WHOL, // 文字集合_全集合
    DENY, // 文字集合_補集合
    ESC,  // エスケープ
}

impl Token {
    pub fn value<'a>(&self) -> &'a str {
        match self {
            Token::SP_S => "(",
            Token::SP_E => ")",
            Token::AND => "&",
            Token::OR => "|",
            Token::INVT => "!",
            Token::REPT => "+",
            Token::CH_S => "[",
            Token::CH_E => "]",
            Token::WHOL => ".",
            Token::DENY => "^",
            Token::ESC => "\\",
        }
    }
}
