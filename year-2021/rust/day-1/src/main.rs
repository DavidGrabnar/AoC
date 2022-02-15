use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "input.txt";

fn main() -> io::Result<()> {
    let values = parse_values();

    println!("By 1: {}\nBy 3: {}", increase_count(&values), increase_count(&group_values(&values)));
    Ok(())
}

fn parse_values() -> Vec<i32> {
    BufReader::new(File::open(INPUT_FILE).unwrap())
        .lines()
        .map(
            |raw| raw.unwrap().parse::<>().unwrap()
        )
        .collect::<Vec<i32>>()
}

fn group_values(values: &[i32]) -> Vec<i32> {
    values
        .windows(3)
        .map(|opts| opts.iter().sum())
        .collect::<Vec<i32>>()
}

fn increase_count(values: &[i32]) -> i32 {
    values
        .windows(2)
        .filter(|opts| opts[1] > opts[0])
        .count() as i32
}
