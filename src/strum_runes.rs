use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::output::print_separator;

#[derive(Debug, Display, EnumIter, EnumString, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
enum Rune {
    Ember,
    Tide,
    Moss,
}

pub fn run() {
    println!("Strum enum example");

    for rune in Rune::iter() {
        println!("Known rune: {rune}");
    }

    let chosen = "tide".parse::<Rune>().expect("known rune should parse");
    println!("Parsed rune: {chosen}");
    print_separator();
}
