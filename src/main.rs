use core::fmt;
use std::collections::HashSet;
use std::fmt::Display;
use std::ops::{BitAnd, BitOr, Not};

enum Token {
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

#[derive(Debug)]
struct Chex {
    kind: u32,
    len: usize,
    include_flg: bool, // falseなら補集合
    char_set: HashSet<char>,
    str: String,
}

impl Chex {
    pub fn new(chars: Vec<char>, include_flg: bool) -> Self {
        let char_set = HashSet::from(chars.into_iter().collect());
        let len = char_set.len();
        let kind = if len == 0 {
            if include_flg {
                0 // 空集合
            } else {
                1 // 全集合
            }
        } else {
            2 // それ以外？
        };

        let str = match kind {
            0 => format!("{}{}", Token::CH_S.value(), Token::CH_E.value()),
            1 => format!("{}", Token::WHOL.value()),
            2 => {
                let mut vec_char = char_set.clone().into_iter().collect::<Vec<char>>();
                vec_char.sort();
                let joind_chars: String = vec_char.into_iter().collect();

                if include_flg && len == 1 {
                    format!("{}", joind_chars)
                } else {
                    let prefix = if include_flg { "" } else { Token::DENY.value() };
                    format!(
                        "{}{}{}{}",
                        Token::CH_S.value(),
                        prefix,
                        joind_chars,
                        Token::CH_E.value()
                    )
                }
            }
            _ => unreachable!(),
        };

        Self {
            kind,
            len,
            char_set,
            include_flg,
            str,
        }
    }

    pub fn new_blank() -> Self {
        Self::new(vec![], true)
    }

    pub fn new_whole() -> Self {
        Self::new(vec![], false)
    }

    pub fn blank(&self) -> bool {
        if self.kind == 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn whole(&self) -> bool {
        if self.kind == 1 {
            return true;
        } else {
            return false;
        }
    }

    pub fn include(&self, other: Self) -> bool {
        match self.kind {
            0 => false,
            1 => true,
            2 => match other.kind {
                0 => true,
                1 => false,
                2 => {
                    if self.include_flg && !other.include_flg {
                        false
                    } else {
                        (!self & other).blank()
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

// impl Display for Chex {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         if let Some(ref str) = self.str {
//             f.write_str(&format!("{}", str))?;
//             Ok(())
//         } else {
//             match self.kind {
//                 0 => self.str = Some(format!("{}{}", Token::CH_S.value(), Token::CH_E.value())),
//                 1 => {}
//                 2 => {}
//                 _ => unreachable!(),
//             }
//             Ok(())
//         }
//         // if self.str == None {
//         //     Ok(())
//         // } else {
//         //     let hoge = self.str.unwrap();
//         //     f.write_str(&format!("{}", ""))?;
//         //     return Ok(());
//         // }
//     }
// }

impl Not for &Chex {
    type Output = Chex;

    fn not(self) -> Self::Output {
        if self.kind == 2 {
            Chex::new(
                self.char_set.clone().into_iter().collect(),
                !self.include_flg,
            )
        } else {
            if self.kind == 0 {
                Chex::new_whole()
            } else if self.kind == 1 {
                Chex::new_blank()
            } else {
                unreachable!()
            }
        }
    }
}

impl PartialEq for Chex {
    fn eq(&self, other: &Self) -> bool {
        if self.kind == other.kind {
            if self.kind != 2 {
                return true;
            } else {
                return self.str == other.str;
            }
        } else {
            return false;
        }
    }
}

impl BitOr for Chex {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        match self.kind {
            0 => other,
            1 => self,
            2 => match other.kind {
                0 => self,
                1 => other,
                2 => {
                    if self.include_flg {
                        if other.include_flg {
                            let chars = self.char_set.union(&other.char_set).cloned().collect();
                            return Self::new(chars, true);
                        } else {
                            let chars =
                                other.char_set.difference(&self.char_set).cloned().collect();
                            return Self::new(chars, false);
                        }
                    } else {
                        if other.include_flg {
                            let chars =
                                self.char_set.difference(&other.char_set).cloned().collect();
                            return Self::new(chars, false);
                        } else {
                            let chars = self
                                .char_set
                                .intersection(&other.char_set)
                                .cloned()
                                .collect();
                            return Self::new(chars, true);
                        }
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl BitAnd for Chex {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        match self.kind {
            0 => self,
            1 => other,
            2 => match other.kind {
                0 => other,
                1 => self,
                2 => {
                    if self.include_flg {
                        if other.include_flg {
                            let chars = self
                                .char_set
                                .intersection(&other.char_set)
                                .cloned()
                                .collect();
                            return Self::new(chars, true);
                        } else {
                            let chars =
                                self.char_set.difference(&other.char_set).cloned().collect();
                            return Self::new(chars, true);
                        }
                    } else {
                        if other.include_flg {
                            let chars =
                                other.char_set.difference(&self.char_set).cloned().collect();
                            return Self::new(chars, true);
                        } else {
                            let chars = other.char_set.union(&self.char_set).cloned().collect();
                            return Self::new(chars, false);
                        }
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("{:#?}", Chex::new(vec!['a', 'b', 'f', 'c'], false));
}

#[cfg(test)]

mod chex_tests {
    use super::*;

    #[test]
    fn blank() {
        let blank = Chex::new_blank();
        assert_eq!("[]", blank.str);
    }

    #[test]
    fn whole() {
        let whole = Chex::new_whole();
        assert_eq!(".", whole.str);
    }

    #[test]
    fn chex() {
        let chex = Chex::new(vec!['a', 'c', 'b'], true);
        assert_eq!("[abc]", chex.str);
    }

    #[test]
    fn invert_chex() {
        let chex = Chex::new(vec!['a', 'c', 'b'], false);
        assert_eq!("[^abc]", chex.str);
    }

    #[test]
    fn not_chex() {
        let chex = Chex::new(vec!['a', 'c', 'b'], true);
        assert_eq!("[^abc]", (!&chex).str);
    }

    #[test]
    fn not_invert_chex() {
        let invert_chex = Chex::new(vec!['a', 'c', 'b'], false);
        assert_eq!("[abc]", (!&invert_chex).str);
    }

    #[test]
    fn intersection_chex() {
        let a = Chex::new(vec!['a', 'c', 'b'], true);
        let b = Chex::new(vec!['a', 'd', 'b'], true);
        assert_eq!("[ab]", (a & b).str);
    }

    #[test]
    fn union_chex() {
        let a = Chex::new(vec!['a', 'c', 'b'], true);
        let b = Chex::new(vec!['a', 'd'], true);
        assert_eq!("[abcd]", (a | b).str);
    }

    #[test]
    fn not_intersection_chex() {
        let a = Chex::new(vec!['a'], false);
        let b = Chex::new(vec!['a', 'd', 'b'], true);
        assert_eq!("[bd]", (a & b).str);
    }

    #[test]
    fn not_union_chex() {
        let a = Chex::new(vec!['a', 'c', 'b'], false);
        let b = Chex::new(vec!['a', 'd'], true);
        assert_eq!("[^bc]", (a | b).str);
    }
}
