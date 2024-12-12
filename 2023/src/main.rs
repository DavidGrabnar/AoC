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
    // println!("Result 2: {}", days::day05::run2(&input)); // 46294175

    // day6
    // let input = fs::read_to_string(Path::new("inputs\\day06-1.txt")).unwrap();
    // println!("Result: {}", days::day06::run(&input)); // 252000
    // println!("Result 2: {}", days::day06::run2(&input)); // 36992486

    // day7
    // let input = fs::read_to_string(Path::new("inputs\\day07-1.txt")).unwrap();
    // println!("Result: {}", days::day07::run(&input)); // 250370104
    // println!("Result 2: {}", days::day07::run2(&input)); // 251735672

    // day8
    // let input = fs::read_to_string(Path::new("inputs\\day08-1.txt")).unwrap();
    // println!("Result: {}", days::day08::run(&input)); // 20513
    // println!("Result 2: {}", days::day08::run2(&input)); // ???

    // day9
    // let input = fs::read_to_string(Path::new("inputs\\day09-1.txt")).unwrap();
    // println!("Result: {}", days::day09::run(&input)); // 1485024777 too high
    // println!("Result 2: {}", days::day08::run2(&input)); //

    // day11
    // let input = fs::read_to_string(Path::new("inputs\\day10-1.txt")).unwrap();
    // println!("Result: {}", days::day10::run(&input, 2)); // 9565386
    // println!("Result 2: {}", days::day11::run(&input, 1_000_000)); // 857986849428

    // day13
    // let input = fs::read_to_string(Path::new("inputs\\day13-1.txt")).unwrap();
    // println!("Result: {}", days::day13::run(&input)); // 33195
    // println!("Result 2: {}", days::day13::run2(&input)); // 40264 too high

    // day14
    // let input = fs::read_to_string(Path::new("inputs\\day14-1.txt")).unwrap();
    // println!("Result: {}", days::day14::run(&input)); //
    // println!("Result 2: {}", days::day14::run2(&input)); //

    // day18
    let input = fs::read_to_string(Path::new("inputs\\day18-1.txt")).unwrap();
    println!("Result: {}", days::day18::run(&input)); // 87847 too high
    // println!("Result 2: {}", days::day18::run2(&input)); //
}
