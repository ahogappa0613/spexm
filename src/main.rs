pub mod chex;
pub mod parser;
pub mod spex;
pub mod token;

use chex::Chex;

use crate::{parser::parse, parser::tokenize, spex::Spex};

fn main() {
    println!("{:#?}", parse(&tokenize("(abc|def|ghi)")));
}
