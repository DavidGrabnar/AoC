use std::fs;
use std::path::Path;

mod days;

fn main() {
    // day1
    // let input = fs::read_to_string(Path::new("inputs\\day01-1.txt")).unwrap();
    // println!("Result: {}", days::day01::run(&input));

    // day2
    let input = fs::read_to_string(Path::new("inputs\\day02-1.txt")).unwrap();
    let input_1 = "12 red, 13 green, 14 blue";
    println!("Result: {}", days::day02::run(&input, input_1));
    println!("Result 2: {}", days::day02::run2(&input));
}
