use std::fmt::{Display, Formatter, write};
use itertools::Itertools;

pub fn run(input: &str, space_size: u32) -> u128 {
    let grid: Grid = input.into();
    let grid = grid.expand(space_size);

    let p = grid.pairs().iter().map(|p| grid.distance(&p)).collect::<Vec<_>>();
    p.iter().sum()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty(u32, u32),
    Galaxy,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty(1, 1),
            '#' => Self::Galaxy,
            _ => panic!("invalid cell {}", value)
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty(x, _) => write!(f, "{}", (0..*x).map(|_|".").join("")),
            Cell::Galaxy => write!(f, "#")
        }
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<Cell>>);

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Grid(
            value.lines().map(|row| row.chars().map(|c| c.into()).collect()).collect()
        )
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|row| {
            let r = row.iter().map(|c| c.to_string()).join("");
            (0..match row.first().unwrap() {
                Cell::Empty(_, y) => *y,
                Cell::Galaxy => 1
            }).map(|_|r.clone())
                .join("\n")
        }).join("\n"))
    }
}

impl Grid {
    pub fn expand(&self, space_size: u32) -> Self {
        let empty_rows = self.0.iter().enumerate()
            .filter(|(_, row)| row.iter().all(|c| match *c {
                Cell::Empty(_, _) => true,
                Cell::Galaxy => false
            }))
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();

        let row_len = self.0.first().unwrap().len();
        let transmuted = self.0.iter().fold(
            (0..row_len).map(|_| vec![]).collect::<Vec<Vec<Cell>>>(),
            |mut out, row| {
                out.iter_mut().enumerate()
                    .for_each(
                        |(idx, t_row)| t_row.push(*row.get(idx).unwrap())
                    );
                out
            });

        let empty_cols = transmuted.iter().enumerate()
            .filter(|(_, row)| row.iter().all(|c|  match *c {
                Cell::Empty(_, _) => true,
                Cell::Galaxy => false
            }))
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();

        let mut clone = self.0.clone();
        clone.iter_mut().for_each(|row|
            empty_cols.iter().rev().for_each(|col_idx| {
                row.remove(*col_idx);
                row.insert(*col_idx, Cell::Empty(space_size, 1))
            })
        );

        let first_row = clone.first().unwrap();
        let row_len = first_row.len();
        // vec![Cell::Empty(1, space_size); row_len]
        let empty_row = (0..row_len).map(|idx| match first_row.get(idx).unwrap() {
            Cell::Empty(x, _) => Cell::Empty(*x, space_size),
            Cell::Galaxy => Cell::Empty(1, space_size)
        }).collect::<Vec<_>>();

        empty_rows.iter().rev().for_each(|row_idx| {
            clone.remove(*row_idx);
            clone.insert(*row_idx, empty_row.clone())
        });

        Self(clone)
    }

    pub fn galaxies(&self) -> Vec<(u32, u32)> {
        self.0.iter()
            .enumerate()
            .map(|(y, row)|
                row.iter().enumerate().filter(|(_, c)| **c == Cell::Galaxy).map(|(x, c)| (x as u32, y as u32)).collect::<Vec<_>>()
            )
            .flatten()
            .collect::<Vec<_>>()
    }

    pub fn pairs(&self) -> Vec<(u32, u32)> {
        let num_galaxies = self.galaxies().len() as u32;

        (1..=num_galaxies).map(|id1| (id1+1..=num_galaxies).map(move |id2| (id1, id2))).flatten().collect()
    }

    pub fn distance(&self, pair: &(u32, u32)) -> u128 {
        let galaxies = self.galaxies();
        let start = galaxies.get(pair.0 as usize - 1).unwrap();
        let end = galaxies.get(pair.1 as usize - 1).unwrap();

        let min_x = start.0.min(end.0);
        let max_x = start.0.max(end.0);
        let x = ((min_x + 1)..=max_x)
            .map(|x| match self.0.get(start.1 as usize).unwrap().get(x as usize).unwrap() {
                Cell::Empty(x, _) => *x as u128,
                Cell::Galaxy => 1u128
            })
            .collect::<Vec<_>>();


        let min_y = start.1.min(end.1);
        let max_y = start.1.max(end.1);
        let y = ((min_y + 1)..=max_y)
            .map(|y| match self.0.get(y as usize).unwrap().get(end.0 as usize).unwrap() {
                Cell::Empty(_, y) => *y as u128,
                Cell::Galaxy => 1u128
            })
            .collect::<Vec<_>>();

        // println!("{:?}, {:?}, {:?} {:?}", start, end, x, y);
        // start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
        x.iter().sum::<u128>() + y.iter().sum::<u128>()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day11::{Grid, run};

    const INPUT_1: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    const PARTIAL_1: &str = r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

    #[test]
    fn test_1_expand() {
        println!("{:?}", Grid::from(INPUT_1).expand(2));
        println!("{}", Grid::from(INPUT_1).expand(2));
        assert_eq!(Grid::from(INPUT_1).expand(2).to_string(), PARTIAL_1);
    }

    #[test]
    fn test_1_galaxies() {
        let galaxies: Vec<(u32, u32)> = vec![
            (3, 0), (7, 1), (0, 2), (6, 4), (1, 5), (9, 6), (7, 8), (0, 9), (4, 9)
        ];

        assert_eq!(Grid::from(INPUT_1).expand(2).galaxies(), galaxies);
    }

    #[test]
    fn test_1_pairs() {
        let pairs: Vec<(u32, u32)> = vec![
            (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (1, 9),
            (2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8), (2, 9),
            (3, 4), (3, 5), (3, 6), (3, 7), (3, 8), (3, 9),
            (4, 5), (4, 6), (4, 7), (4, 8), (4, 9),
            (5, 6), (5, 7), (5, 8), (5, 9),
            (6, 7), (6, 8), (6, 9),
            (7, 8), (7, 9),
            (8, 9),
        ];

        assert_eq!(Grid::from(INPUT_1).expand(2).pairs(), pairs);
    }

    #[test]
    fn test_1_distance() {
        assert_eq!(Grid::from(INPUT_1).expand(2).distance(&(5, 9)), 9);
        assert_eq!(Grid::from(INPUT_1).expand(2).distance(&(1, 7)), 15);
        assert_eq!(Grid::from(INPUT_1).expand(2).distance(&(3, 6)), 17);
        assert_eq!(Grid::from(INPUT_1).expand(2).distance(&(8, 9)), 5);
        assert_eq!(Grid::from(INPUT_1).expand(2).distance(&(1, 2)), 6);
    }

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1, 2), 374);
        assert_eq!(run(INPUT_1, 10), 1030);
        assert_eq!(run(INPUT_1, 100), 8410);
    }
}