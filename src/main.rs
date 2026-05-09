mod itertools_groups;
mod mandelbrot_ascii;
mod output;
mod parallel_numbers;
mod quest_json;
mod serde_spell;
mod strum_runes;

fn main() {
    println!("Alchemy demos");
    println!();

    itertools_groups::run();
    strum_runes::run();
    parallel_numbers::run();
    serde_spell::run();
    mandelbrot_ascii::run();
    quest_json::run();
}
