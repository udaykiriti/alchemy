mod itertools_groups;
mod mandelbrot_ascii;
mod output;
mod parallel_numbers;
mod quest_json;
mod serde_spell;
mod strum_runes;

struct Demo {
    name: &'static str,
    run: fn(),
}

fn main() {
    output::print_section("Alchemy demos");

    let demos = [
        Demo {
            name: "Itertools groups",
            run: itertools_groups::run,
        },
        Demo {
            name: "Strum runes",
            run: strum_runes::run,
        },
        Demo {
            name: "Parallel numbers",
            run: parallel_numbers::run,
        },
        Demo {
            name: "Serde spell",
            run: serde_spell::run,
        },
        Demo {
            name: "Mandelbrot ASCII",
            run: mandelbrot_ascii::run,
        },
        Demo {
            name: "Quest JSON",
            run: quest_json::run,
        },
    ];

    for (index, demo) in demos.iter().enumerate() {
        println!("{}. {}", index + 1, demo.name);
        (demo.run)();
    }
}
