use rayon::prelude::*;

use crate::output::{print_section, print_separator};

pub fn run() {
    print_section("Parallel number transform");

    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let scaled_values: Vec<_> = values.par_iter().map(|value| value * 10).collect();

    println!("Scaled values: {scaled_values:?}");
    print_separator();
}
