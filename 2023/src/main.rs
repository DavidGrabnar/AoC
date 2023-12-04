use std::fs;
use std::path::Path;

mod days;

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string(Path::new("inputs\\day01-1.txt")).unwrap();
    println!("Result: {}", days::day01::run(&input));
}
