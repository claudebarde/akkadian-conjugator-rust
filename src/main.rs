use std::env::{args, Args};
pub mod verb_finder;
pub mod conjugator;

fn main() {
    let mut args: Args = args();
    let verb = args.nth(1).unwrap();
    let verb_data = verb_finder::find_verb(&verb);
    println!("{:?}", verb_data);
    let conjugated_verb = conjugator::conjugate(&verb, &verb_data);
    println!("{:#?}", conjugated_verb);
}
