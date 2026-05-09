use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Spell {
    name: String,
    mana_cost: u32,
    restricted: bool,
}

pub fn run() {
    println!("Serialize a struct");

    let spell = Spell {
        name: String::from("Fireball"),
        mana_cost: 50,
        restricted: false,
    };

    let json = serde_json::to_string_pretty(&spell).expect("spell should serialize");
    println!("JSON:\n{json}");

    let restored: Spell = serde_json::from_str(&json).expect("json should deserialize");
    println!("Round-tripped value: {restored:?}");
    print_separator();
}

fn print_separator() {
    println!("--------------------------------------------------");
}
