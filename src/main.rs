use std::env::{args, Args};
pub mod verb_finder;
pub mod conjugator;
pub mod utils;

fn main() {
    let mut args: Args = args();
    let verb = args.nth(1).unwrap();
    let verb_data = verb_finder::find_verb(&verb);
    let conjugated_verb = conjugator::conjugate(&verb, &verb_data);
    println!("{:#?}", conjugated_verb);
}
