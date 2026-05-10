use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::output::{print_section, print_separator};

#[derive(Serialize, Deserialize, Debug)]
struct Spell {
    name: Cow<'static, str>,
    mana_cost: u32,
    restricted: bool,
}

pub fn run() {
    print_section("Serialize a struct");

    let spell = Spell {
        name: Cow::Borrowed("Fireball"),
        mana_cost: 50,
        restricted: false,
    };

    let json = serde_json::to_string_pretty(&spell).expect("spell should serialize");
    println!("JSON:\n{json}");

    let restored: Spell = serde_json::from_str(&json).expect("json should deserialize");
    println!("Round-tripped value: {restored:?}");
    print_separator();
}
