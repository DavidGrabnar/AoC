
pub fn run(input: &str) -> i32 {
    let x = input.lines()
        .map(|row| Series(row.split_whitespace().map(|el | el.parse().unwrap()).collect()))
        .map(|s| s.complete())
        .collect::<Vec<_>>();

    println!("{:?}", x.get(2).unwrap());
    println!("{:?}", input.lines().skip(2).next().unwrap().split_whitespace().map(|el | el.parse::<i32>().unwrap()).collect::<Vec<_>>());

    x.iter().map(|ss| ss.iter().map(|s| s.0.last().unwrap()).sum::<i32>())
        .sum::<i32>()
}

pub fn run2(input: &str) -> i32 {
    return 0;
}

#[derive(Debug, Clone)]
struct Series(Vec<i32>);

impl Series {
    fn reduced(&self) -> Self {
        Series(
            self.0.iter().zip(self.0.clone().iter().skip(1))
                .map(|(l, r)| r - l)
                .collect()
        )
    }

    fn complete(&self) -> Vec<Self> {
        let mut out = vec![];
        let mut last = self.clone();
        while last.0.iter().sum::<i32>() > 0 {
            out.push(last.clone());
            last = last.reduced();
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day09::{run, run2};

    const INPUT_1: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 114);
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
        assert_eq!(run2(INPUT_2), 281);
    }
}