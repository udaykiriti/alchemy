mod glyph_strum;
mod potion_rayon;
mod quest_archive;
mod satchel_itertools;
mod scroll_serde;
mod star_forge;

fn main() {
    println!("Alchemy demos");
    println!();

    satchel_itertools::run();
    glyph_strum::run();
    potion_rayon::run();
    scroll_serde::run();
    star_forge::run();
    quest_archive::run();
}
