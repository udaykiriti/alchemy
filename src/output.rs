const SEPARATOR: &str = "--------------------------------------------------";

pub fn print_section(title: &str) {
    println!("{title}");
    println!();
}

pub fn print_separator() {
    println!("{SEPARATOR}");
}
