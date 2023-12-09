use std::fs;
use std::path::Path;

mod days;

fn main() {
    // day1
    // let input = fs::read_to_string(Path::new("inputs\\day01-1.txt")).unwrap();
    // println!("Result: {}", days::day01::run(&input));

    // day2
    // let input = fs::read_to_string(Path::new("inputs\\day02-1.txt")).unwrap();
    // let input_1 = "12 red, 13 green, 14 blue";
    // println!("Result: {}", days::day02::run(&input, input_1));
    // println!("Result 2: {}", days::day02::run2(&input));

    // day3
    // let input = fs::read_to_string(Path::new("inputs\\day03-1.txt")).unwrap();
    // println!("Result: {}", days::day03::run(&input));
    // println!("Result 2: {}", days::day03::run2(&input));

    // day4
    // let input = fs::read_to_string(Path::new("inputs\\day04-1.txt")).unwrap();
    // println!("Result: {}", days::day04::run(&input));
    // println!("Result 2: {}", days::day04::run2(&input));

    // day5
    // let input = fs::read_to_string(Path::new("inputs\\day05-1.txt")).unwrap();
    // println!("Result: {}", days::day05::run(&input)); // 484023871
    // println!("Result 2: {}", days::day05::run2(&input));

    // day6
    let input = fs::read_to_string(Path::new("inputs\\day06-1.txt")).unwrap();
    println!("Result: {}", days::day06::run(&input)); // 252000
    println!("Result 2: {}", days::day06::run2(&input)); // 36992486
}
