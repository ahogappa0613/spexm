use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::ops::{BitAnd, BitOr, Not};

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Blank,
    Whole,
    Other,
}

#[derive(Debug, Clone)]
pub struct Chex {
    pub kind: Kind,
    pub len: usize,
    pub include_flg: bool, // falseなら補集合
    pub char_set: HashSet<char>,
    pub str: String,
}

impl Chex {
    pub fn new(chars: Vec<char>, include_flg: bool) -> Self {
        let char_set = HashSet::from(chars.into_iter().collect());
        let len = char_set.len();
        let kind = if len == 0 {
            if include_flg {
                Kind::Blank // 空集合
            } else {
                Kind::Whole // 全集合
            }
        } else {
            Kind::Other // それ以外？
        };

        let str = match kind {
            Kind::Blank => format!("{}{}", Token::CH_S.value(), Token::CH_E.value()),
            Kind::Whole => format!("{}", Token::WHOL.value()),
            Kind::Other => {
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
        match self.kind {
            Kind::Blank => true,
            _ => false,
        }
    }

    pub fn whole(&self) -> bool {
        match self.kind {
            Kind::Whole => true,
            _ => false,
        }
    }

    pub fn include(&self, other: &Self) -> bool {
        match self.kind {
            Kind::Blank => false,
            Kind::Whole => true,
            Kind::Other => match other.kind {
                Kind::Blank => true,
                Kind::Whole => false,
                Kind::Other => {
                    if self.include_flg && !other.include_flg {
                        false
                    } else {
                        (&!self & other).blank()
                    }
                }
            },
        }
    }
}

impl Display for Chex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{}", self.str))?;
        Ok(())
    }
}

impl Not for &Chex {
    type Output = Chex;

    fn not(self) -> Self::Output {
        match self.kind {
            Kind::Blank => Chex::new_whole(),
            Kind::Whole => Chex::new_whole(),
            Kind::Other => Chex::new(
                self.char_set.clone().into_iter().collect(),
                !self.include_flg,
            ),
        }
    }
}

impl PartialEq for Chex {
    fn eq(&self, other: &Self) -> bool {
        if self.kind == other.kind {
            if self.kind != Kind::Other {
                return true;
            } else {
                return self.str == other.str;
            }
        } else {
            return false;
        }
    }
}

impl BitOr for &Chex {
    type Output = Chex;

    fn bitor(self, other: Self) -> Self::Output {
        match self.kind {
            Kind::Blank => other.clone(),
            Kind::Whole => self.clone(),
            Kind::Other => match other.kind {
                Kind::Blank => self.clone(),
                Kind::Whole => other.clone(),
                Kind::Other => {
                    if self.include_flg {
                        if other.include_flg {
                            let chars = self.char_set.union(&other.char_set).cloned().collect();
                            return Chex::new(chars, true);
                        } else {
                            let chars =
                                other.char_set.difference(&self.char_set).cloned().collect();
                            return Chex::new(chars, false);
                        }
                    } else {
                        if other.include_flg {
                            let chars =
                                self.char_set.difference(&other.char_set).cloned().collect();
                            return Chex::new(chars, false);
                        } else {
                            let chars = self
                                .char_set
                                .intersection(&other.char_set)
                                .cloned()
                                .collect();
                            return Chex::new(chars, true);
                        }
                    }
                }
            },
        }
    }
}

impl BitAnd for &Chex {
    type Output = Chex;

    fn bitand(self, other: Self) -> Self::Output {
        match self.kind {
            Kind::Blank => self.clone(),
            Kind::Whole => other.clone(),
            Kind::Other => match other.kind {
                Kind::Blank => other.clone(),
                Kind::Whole => self.clone(),
                Kind::Other => {
                    if self.include_flg {
                        if other.include_flg {
                            let chars = self
                                .char_set
                                .intersection(&other.char_set)
                                .cloned()
                                .collect();
                            return Chex::new(chars, true);
                        } else {
                            let chars =
                                self.char_set.difference(&other.char_set).cloned().collect();
                            return Chex::new(chars, true);
                        }
                    } else {
                        if other.include_flg {
                            let chars =
                                other.char_set.difference(&self.char_set).cloned().collect();
                            return Chex::new(chars, true);
                        } else {
                            let chars = other.char_set.union(&self.char_set).cloned().collect();
                            return Chex::new(chars, false);
                        }
                    }
                }
            },
        }
    }
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
        let ref a = Chex::new(vec!['a', 'c', 'b'], true);
        let ref b = Chex::new(vec!['a', 'd', 'b'], true);
        assert_eq!("[ab]", (a & b).str);
    }

    #[test]
    fn union_chex() {
        let ref a = Chex::new(vec!['a', 'c', 'b'], true);
        let ref b = Chex::new(vec!['a', 'd'], true);
        assert_eq!("[abcd]", (a | b).str);
    }

    #[test]
    fn not_intersection_chex() {
        let ref a = Chex::new(vec!['a'], false);
        let ref b = Chex::new(vec!['a', 'd', 'b'], true);
        assert_eq!("[bd]", (a & b).str);
    }

    #[test]
    fn not_union_chex() {
        let ref a = Chex::new(vec!['a', 'c', 'b'], false);
        let ref b = Chex::new(vec!['a', 'd'], true);
        assert_eq!("[^bc]", (a | b).str);
    }
}
