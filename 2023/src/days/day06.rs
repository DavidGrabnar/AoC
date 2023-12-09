use std::collections::HashMap;
use std::ops::Div;
use regex::{Regex};

pub fn run(input: &str) -> u128 {
    let records = Records::new(input);
    println!("{:?}", records);
    records.0.iter()
        .map(|el| el.options())
        .map(|[(min_v, _), (max_v, _)]| max_v - min_v + 1)
        .reduce(|acc, el| acc * el)
        .unwrap()
}

pub fn run2(input: &str) -> u128 {
    let records = Records::new(&input.replace(" ", ""));
    println!("{:?}", records);
    let [(min_v, _), (max_v, _)] = records.0.iter().next().unwrap().options();
    max_v - min_v + 1
}

#[derive(Debug)]
struct Record(u128, u128);

impl Record {
    // fn new(input: &str) -> Self {
    //     let mut lines = input.replace(" ", "");
    //     println!("")
    // }

    fn options(&self) -> [(u128, u128); 2] { // (hold = velocity, distance) x 2 = min, max
        // exact -> solve for:
        // dist = time*hold - hold^2
        // results in h1, h2
        // take hmin, round up
        let time = self.0 as f64;
        let dist = self.1 as f64;
        // min is negative, so we abs
        let exact = (- time + (time.powf(2f64) - 4f64 * dist).sqrt()).div(2f64).abs();
        let mut whole = exact.ceil();

        if exact == whole {
            whole += 1f64;
        }

        let whole = whole.abs() as u128;
        println!("END {} {} {} {}", time, dist, exact, whole);
        [
            (whole, self.0 - whole),
            (self.0 - whole, whole)
        ]
    }
}

#[derive(Debug)]
struct Records(Vec<Record>);

impl Records {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();

        let mut time_raw = lines.next().unwrap().split(":");
        time_raw.next();
        let time = time_raw.next().unwrap().trim()
            .split_whitespace()
            .map(|r| r.parse::<u128>().unwrap());

        let mut dist_raw = lines.next().unwrap().split(":");
        dist_raw.next();
        let dist = dist_raw.next().unwrap().trim()
            .split_whitespace()
            .map(|r| r.parse::<u128>().unwrap());

        Records(
            time.zip(dist)
                .map(|(time, dist)| Record(time, dist))
                .collect()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day06::{run, run2};

    const INPUT_1: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 288);
    }

    #[test]
    fn test_2() {
        assert_eq!(run2(INPUT_1), 71503);
    }
}