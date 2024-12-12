pub fn run(input: &str) -> i32 {
    let grid: Grid = input.into();
    println!("{:?}", grid);

    return 0;
}

pub fn run2(input: &str) -> i32 {
    return 0;
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("invalid cell {}", value)
        }
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<Cell>>);

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self (
            value.lines()
            .map(|row| row.chars().map(|c|c.into()).collect())
            .collect()
        )
    }

    /*fn start_pos(&self) -> (u32, u32) {
        self.0.iter().enumerate()
            .find
    }*/
}

#[cfg(test)]
mod tests {
    use crate::days::day10::{run, run2};

    const INPUT_1: &str = r#".....
.S-7.
.|.|.
.L-J.
....."#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 4);
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