use rayon::prelude::*;

const WIDTH: usize = 72;
const HEIGHT: usize = 24;
const MAX_ITERATIONS: u32 = 40;
const PALETTE: [char; 10] = [' ', '.', ',', ':', ';', 'o', 'x', '%', '#', '@'];

pub fn run() {
    println!("Parallel Mandelbrot render");

    let rows: Vec<String> = (0..HEIGHT)
        .into_par_iter()
        .map(|y| {
            let imaginary = scale(y, HEIGHT, -1.2, 1.2);

            (0..WIDTH)
                .map(|x| {
                    let real = scale(x, WIDTH, -2.1, 0.9);
                    let iterations = mandelbrot_escape(real, imaginary, MAX_ITERATIONS);
                    let palette_index =
                        (iterations as usize * (PALETTE.len() - 1)) / MAX_ITERATIONS as usize;

                    PALETTE[palette_index]
                })
                .collect()
        })
        .collect();

    for row in rows {
        println!("{row}");
    }

    println!("Render complete.");
    print_separator();
}

fn scale(value: usize, upper_bound: usize, min: f64, max: f64) -> f64 {
    min + (value as f64 / upper_bound as f64) * (max - min)
}

fn mandelbrot_escape(real: f64, imaginary: f64, max_iterations: u32) -> u32 {
    let mut z_real = 0.0;
    let mut z_imaginary = 0.0;
    let mut iterations = 0;

    while z_real * z_real + z_imaginary * z_imaginary <= 4.0 && iterations < max_iterations {
        let next_real = z_real * z_real - z_imaginary * z_imaginary + real;
        z_imaginary = 2.0 * z_real * z_imaginary + imaginary;
        z_real = next_real;
        iterations += 1;
    }

    iterations
}

fn print_separator() {
    println!("--------------------------------------------------");
}
