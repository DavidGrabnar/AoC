use std::collections::HashMap;
use std::error::Error;
use regex::{Regex};

pub fn run(input: &str) -> i32 {
    input
        .split("\n")
        .map(|row| Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)")
            .expect("invalid regex")
            .captures_iter(row)
            .map(|c| c.extract())
            .map(|(_, [v])| Digit::new(v))
            .collect::<Vec<_>>()
        )
        .map(Digit::value)
        .sum()
}

#[derive(Debug, Copy, Clone)]
struct Digit(i32);

const STR_DIGIT_MAP: [(&str, i32); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

impl Digit {
    fn new(input: &str) -> Self {
        HashMap::from(STR_DIGIT_MAP)
            .get(input)
            .map(|v| Digit(*v))
            .unwrap()
    }

    fn value(row: Vec<Digit>) -> i32 {
        row.get(0).expect("no digits in row").0 * 10 + row.last().expect("no digits in row").0
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day01::run;

    const INPUT_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 142);
    }

    const INPUT_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn test_2() {
        assert_eq!(run(INPUT_2), 281);
    }
}