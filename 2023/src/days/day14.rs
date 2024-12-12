
pub fn run(input: &str) -> i32 {
    0
}

#[derive(Debug)]
enum Cell {
    Round,
    Square,
    Empty
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::Round,
            '#' => Self::Square,
            '.' => Self::Empty,
            _ => panic!("invalid cell {}", value)
        }
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<Cell>>);

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self(
            value.lines()
                .map(|row| row.chars().map(|c| c.into()).collect())
                .collect()
        )
    }
}

impl Grid {
    // fn shift_north(&self) -> Grid {
    //     self.0.
    // }
}

#[cfg(test)]
mod tests {
    use crate::days::day14::run;

    const INPUT_1: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    const OUTPUT_1_SHIFT_NORTH: &str = r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 136);
    }
}