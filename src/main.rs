#[macro_use]
extern crate itertools;

mod tournament;

use tournament::Tournament;

fn main() {
    let mut t = Tournament::new();
    t.run_tournament();
    println!("{}", t);
}
