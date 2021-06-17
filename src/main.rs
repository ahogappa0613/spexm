pub mod chex;
pub mod spex;
pub mod token;

use chex::Chex;

fn main() {
    println!("{:#?}", Chex::new(vec!['a', 'b', 'f', 'c'], false));
}
