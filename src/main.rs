pub mod builder;
pub mod chex;
pub mod parser;
pub mod spex;
pub mod token;

use chex::Chex;

use crate::builder::spex;

fn main() {
    println!("{:#?}", spex("((a[bc])+&!((ac)+))|a+").mermaid());
}
