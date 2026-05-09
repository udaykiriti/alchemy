use rayon::prelude::*;

pub fn run() {
    println!("Parallel number transform");

    let mut values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    values.par_iter_mut().for_each(|value| {
        *value *= 10;
    });

    println!("Scaled values: {values:?}");
    print_separator();
}

fn print_separator() {
    println!("--------------------------------------------------");
}
