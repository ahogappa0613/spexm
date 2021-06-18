pub mod chex;
pub mod spex;
pub mod token;

use chex::Chex;

use crate::spex::Spex;

fn main() {
    Spex::new_blank().mermaid();
    println!("{:#?}", Spex::new_blank());
}
