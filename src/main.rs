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
    write!(file, "{}", spex("ab").mermaid()).unwrap();
    file.flush().unwrap();
}
