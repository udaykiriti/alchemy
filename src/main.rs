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

impl Snippet {
    const fn new(name: &'static str, run: fn()) -> Self {
        Self { name, run }
    }
}

fn main() {
    output::print_section("Alchemy demos");

    let snippets = [
        Snippet::new("Itertools groups", itertools_groups::run),
        Snippet::new("Strum runes", strum_runes::run),
        Snippet::new("Parallel numbers", parallel_numbers::run),
        Snippet::new("Serde spell", serde_spell::run),
        Snippet::new("Mandelbrot ASCII", mandelbrot_ascii::run),
        Snippet::new("Quest JSON", quest_json::run),
    ];

    for (index, snippet) in snippets.iter().enumerate() {
        println!("\n{}. {}", index + 1, snippet.name);
        (snippet.run)();
    }
}
