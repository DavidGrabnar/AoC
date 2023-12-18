use std::fmt::{Display, Formatter};
use itertools::Itertools;

pub fn run(input: &str) -> i32 {
    Mirrors::from(Grids::from(input)).value() as i32
}

pub fn run2(input: &str) -> i32 {
    let mut grids = Grids::from(input);
    grids.0.iter_mut().for_each(|grid| grid.swap_cell(grid.find_single_fix()));
    // println!("{:?}", grids);
    // grids.0.iter().for_each(|grid| println!("{:?} {:?}", grid.vertical_mirror(), grid.horizontal_mirror()));
    Mirrors::from(grids).value() as i32
}

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Ash,
    Rock,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("invalid cell {}", value)
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Ash => '.',
            Cell::Rock => '#'
        })
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

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter()
            .map(|row|
                row.iter().map(|cell| cell.to_string()).join("")
            )
            .join("\n"))
    }
}

impl Grid {
    fn vertical_mirror(&self) -> Option<u32> {
        (0..(self.0.get(0).unwrap().len() - 1)).find(|i|
            self.vertical_diffs(*i).is_empty()
        ).map(|el| el as u32 + 1)
    }

    fn vertical_diffs(&self, mirror_x: usize) -> Vec<(usize, usize)> {
        self.0.iter().enumerate()
            .map(|(y, row)|
                (0..(mirror_x + 1).min(row.len() - mirror_x - 1))
                    .filter(|j|
                        row[mirror_x - j] != row[mirror_x + 1 + j]
                    )
                    .map(move |partial_x| (mirror_x - partial_x, y))
            )
            .flatten()
            .collect()
    }

    fn horizontal_mirror(&self) -> Option<u32> {
        (0..(self.0.len() - 1)).find(|i|
            self.horizontal_diffs(*i).is_empty()
        ).map(|el| el as u32 + 1)
    }

    fn horizontal_diffs(&self, mirror_y: usize) -> Vec<(usize, usize)> {
        (0..self.0.get(0).unwrap().len()).enumerate().map(|(x, idx)|
            (0..(mirror_y + 1).min(self.0.len() - mirror_y - 1))
                .filter(move |j|
                    self.0.get(mirror_y - j).unwrap()[idx] != self.0.get(mirror_y + 1 + j).unwrap()[idx]
                )
                .map(move |partial_y| (x, mirror_y - partial_y))
        )
            .flatten()
            .collect()
    }

    fn find_single_fix(&self) -> (usize, usize) {
        let horizontal_fix = (0..self.0.len() - 1)
            .map(|y| self.horizontal_diffs(y))
            .find(|diffs| diffs.len() == 1)
            .map(|diffs| diffs.first().unwrap().clone());

        if let Some(pos) = horizontal_fix {
            return pos;
        }

        let vertical_fix = (0..self.0.get(0).unwrap().len() - 1)
            .map(|x| self.vertical_diffs(x))
            .find(|diffs| diffs.len() == 1)
            .map(|diffs| diffs.first().unwrap().clone());

        if let Some(pos) = vertical_fix {
            return pos;
        }

        panic!("no single fix for grid {:?}", self);
    }

    fn swap_cell(&mut self, cell: (usize, usize)) {
        let (x, y) = cell;
        let row = self.0.get_mut(y).unwrap();

        let old_value = row.remove(x);
        let new_value = match old_value {
            Cell::Ash => Cell::Rock,
            Cell::Rock => Cell::Ash
        };

        row.insert(x, new_value);
    }
}

#[derive(Debug)]
struct Grids(Vec<Grid>);

impl From<&str> for Grids {
    fn from(value: &str) -> Self {
        let (mut groups, group) = value.lines().fold((vec![], vec![]), |(mut groups, mut group), line| {
            if line.is_empty() {
                groups.push(group);
                group = vec![];
            } else {
                group.push(line);
            }

            (groups, group)
        });

        if !group.is_empty() {
            groups.push(group);
        }
        Self(
            groups.iter().map(|group| Grid::from(group.join("\n").as_str()))
                .collect()
        )
    }
}

impl Display for Grids {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|grid| grid.to_string()).join("\n\n"))
    }
}

#[derive(Debug)]
enum Mirror {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
struct Mirrors(Vec<(Mirror, u32)>);

impl Mirrors {
    fn value(&self) -> u32 {
        self.0.iter().map(|(mirror, value)| match mirror {
            Mirror::Vertical => *value,
            Mirror::Horizontal => *value * 100
        }).sum()
    }
}

impl From<Grids> for Mirrors {
    fn from(value: Grids) -> Self {
        Self(
            value.0.iter().map(|grid|
                if let Some(value) = grid.horizontal_mirror() {
                    (Mirror::Horizontal, value)
                } else if let Some(value) = grid.vertical_mirror() {
                    (Mirror::Vertical, value)
                } else {
                    panic!("no mirror in grid {:?}", grid)
                }
            ).collect()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day13::{Grids, run, run2};

    const INPUT_1: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn test_1_vertical() {
        assert_eq!(Grids::from(INPUT_1).0.get(0).unwrap().vertical_mirror(), Some(5));
    }

    #[test]
    fn test_1_horizontal() {
        println!("{}", format!("{:?}", Grids::from(INPUT_1)).replace("]", "]\n"));
        assert_eq!(Grids::from(INPUT_1).0.get(1).unwrap().horizontal_mirror(), Some(4));
    }

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 405);
    }

    const OUTPUT_2_FIXED: &str = r#"..##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#....#..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn test_2_fixed() {
        let mut grids = Grids::from(INPUT_1);
        grids.0.iter_mut().for_each(|grid| grid.swap_cell(grid.find_single_fix()));
        assert_eq!(grids.to_string(), OUTPUT_2_FIXED);
    }

    #[test]
    fn test_2() {
        assert_eq!(run2(INPUT_1), 400);
    }
}