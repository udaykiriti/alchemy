use itertools::Itertools;

struct Ingredient {
    name: &'static str,
    kind: &'static str,
}

pub fn run() {
    println!("Itertools example");

    let satchel = [
        Ingredient {
            name: "mint",
            kind: "herb",
        },
        Ingredient {
            name: "moon salt",
            kind: "mineral",
        },
        Ingredient {
            name: "sage",
            kind: "herb",
        },
        Ingredient {
            name: "amber dust",
            kind: "mineral",
        },
        Ingredient {
            name: "dew drop",
            kind: "liquid",
        },
    ];

    let ordered = satchel.iter().sorted_by_key(|ingredient| ingredient.kind);

    for (kind, items) in &ordered.chunk_by(|ingredient| ingredient.kind) {
        let names = items.map(|ingredient| ingredient.name).sorted().join(", ");
        println!("{kind}: {names}");
    }

    let shopping_line = satchel
        .iter()
        .map(|ingredient| ingredient.name)
        .sorted()
        .join(" | ");
    println!("Packed: {shopping_line}");
    print_separator();
}

fn print_separator() {
    println!("--------------------------------------------------");
}
