use std::collections::HashMap;
use std::str::Lines;

pub fn run(input: &str) -> i32 {
    let mut lines = input.lines();

    let steps = Steps::new(lines.next().unwrap().trim());
    lines.next();

    let dirs = Directions::new(lines);

    // println!("{:?} {:?} {}", steps, dirs, dirs.runs(&steps));
    dirs.runs(&steps) as i32
}

pub fn run2(input: &str) -> i32 {
    let mut lines = input.lines();

    let steps = Steps::new(lines.next().unwrap().trim());
    lines.next();

    let dirs = Directions::new(lines);

    // println!("{:?} {:?} {}", steps, dirs, dirs.runs2(&steps));
    dirs.runs2(&steps) as i32
}

#[derive(Debug)]
enum Step {
    Left,
    Right
}

impl From<char> for Step {
    fn from(value: char) -> Self {
        match value {
            'L' => Step::Left,
            'R' => Step::Right,
            _ => panic!("invalid step {}", value)
        }
    }
}

#[derive(Debug)]
struct Steps(Vec<Step>);

impl Steps {
    fn new(raw: &str) -> Steps {
        Steps(
            raw.chars().map(|c| Step::from(c)).collect()
        )
    }
}

const START: &str = "AAA";
const END: &str = "ZZZ";

#[derive(Debug)]
struct Directions(HashMap<String, (String, String)>);

impl Directions {
    fn new(lines: Lines) -> Self {
        Directions(
            lines.map(|row| {
                let mut split = row.split(" = ");
                let key = split.next().unwrap();

                let clean = split.next().unwrap().replace("(", "").replace(")", "");
                let mut split_dest = clean.split(", ");
                (key.to_string(), (split_dest.next().unwrap().to_string(), split_dest.next().unwrap().to_string()))
            })
                .collect()
        )
    }

    fn runs(&self, steps: &Steps) -> u32 {
        let mut current = START.to_string();
        let mut runs = 0;
        while current != END {
            for step in &steps.0 {
                let value = self.0.get(&current).unwrap().clone();
                current = match step {
                    Step::Left => value.0,
                    Step::Right => value.1
                };
                runs += 1;
            }
        }

        return runs;
    }

    fn runs2(&self, steps: &Steps) -> u32 {
        let mut current = self.0.keys().cloned().filter(|k| k.chars().last().unwrap() == 'A').collect::<Vec<_>>();
        let mut runs = 0;
        while current.iter().any(|k| k.chars().last().unwrap() != 'Z') {
            for step in &steps.0 {
                current = current.into_iter().map(|k| {
                    let value = self.0.get(&k).unwrap().clone();
                    match step {
                        Step::Left => value.0,
                        Step::Right => value.1
                    }
                })
                    .collect();

                runs += 1;
            }
        }

        return runs;
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day08::{run, run2};

    const INPUT_1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 2);
    }

    const INPUT_2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    #[test]
    fn test_2() {
        assert_eq!(run(INPUT_2), 6);
    }

    const INPUT_3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn test_3() {
        assert_eq!(run2(INPUT_3), 6);
    }
}