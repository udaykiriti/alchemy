mod itertools_groups;
mod mandelbrot_ascii;
mod output;
mod parallel_numbers;
mod quest_json;
mod serde_spell;
mod strum_runes;

struct Snippet {
    name: &'static str,
    run: fn(),
}

fn main() {
    output::print_section("Alchemy demos");

    let snippets = [
        Snippet {
            name: "Itertools groups",
            run: itertools_groups::run,
        },
        Snippet {
            name: "Strum runes",
            run: strum_runes::run,
        },
        Snippet {
            name: "Parallel numbers",
            run: parallel_numbers::run,
        },
        Snippet {
            name: "Serde spell",
            run: serde_spell::run,
        },
        Snippet {
            name: "Mandelbrot ASCII",
            run: mandelbrot_ascii::run,
        },
        Snippet {
            name: "Quest JSON",
            run: quest_json::run,
        },
    ];

    for (index, snippet) in snippets.iter().enumerate() {
        println!("\n{}. {}", index + 1, snippet.name);
        (snippet.run)();
    }
}
