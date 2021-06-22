pub mod builder;
pub mod chex;
pub mod parser;
pub mod spex;
pub mod token;

use chex::Chex;
use spex::Spex;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use crate::builder::spex;

fn main() {
    let mut file = File::create("./result.md").unwrap();
    //write!(file, "{}", spex("((a[bc])+&!((ac)+))|a+|((([^def]&[l])|[op])&[^u])+").mermaid()).unwrap();
    write!(file, "{}", spex("((a[bc])+&!((ac)+))|a+").mermaid());
    file.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(spex("(a+|ab)&!a") == spex("a(a+|b)"), true);
        assert_eq!(spex("(a+|ab)") == spex("a(a+|b)"), false);
        assert_eq!(spex("(a+|ab)").include(&spex("a(a+|b)")), true);
        assert_eq!(!(&spex("[abc]+") & &spex("ababca")).blank(), true);
        assert_eq!(!(&spex("(abc)+") & &spex("....a|....a.+")).blank(), false);
        assert_eq!(!(&spex("(abc)+") & &spex("....b|....b.+")).blank(), true);
    }
}
