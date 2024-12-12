use std::fmt::{Display, Formatter};
use itertools::Itertools;

pub fn run(input: &str) -> u32 {
    let mut outline = Plan::from(input).outline_trench();
    println!("{}", outline);
    outline.fill();
    println!("{}", outline);
    outline.area()
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Step(Dir, u32);

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let mut split = value.split_whitespace();
        Self(
            split.next().unwrap().chars().next().unwrap().into(),
            split.next().unwrap().parse::<u32>().unwrap(),
        )
    }
}

#[derive(Debug)]
struct Plan(Vec<Step>);

impl From<&str> for Plan {
    fn from(value: &str) -> Self {
        Self(
            value.lines()
                .map(|row| row.into())
                .collect()
        )
    }
}

impl Plan {
    fn outline_trench(&self) -> Map {
        let mut map = Map(vec![vec![Cell::Trench]]);
        let mut size = (1i32, 1i32);
        let mut curr = (0i32, 0i32);

        self.0.iter().for_each(|step| {
            let mut next: (i32, i32) = match step.0 {
                Dir::Up => (curr.0, curr.1 - step.1 as i32),
                Dir::Down => (curr.0, curr.1 + step.1 as i32),
                Dir::Left => (curr.0 - step.1 as i32, curr.1),
                Dir::Right => (curr.0 + step.1 as i32, curr.1),
            };

            // resize
            (next.0..0).for_each(|_| {
                map.0.iter_mut().for_each(|row| row.insert(0, Cell::Terrain));
            });

            if next.0 < 0 {
                curr.0 -= next.0;
            }

            let diff_x = next.0 - size.0;
            (0..=diff_x).for_each(|_| {
                map.0.iter_mut().for_each(|row| row.push(Cell::Terrain));
            });
            size.0 = map.0.first().unwrap().len() as i32;

            (next.1..0).for_each(|_| {
                map.0.insert(0, vec![Cell::Terrain; size.0 as usize]);
            });

            if next.1 < 0 {
                curr.1 -= next.1;
            }

            let diff_y = next.1 - size.1;
            (0..=diff_y).for_each(|_| {
                map.0.push(vec![Cell::Terrain; size.0 as usize]);
            });
            size.1 = map.0.len() as i32;

            // outline
            (0..step.1).for_each(|idx| {
                match step.0 {
                    Dir::Up => curr.1 -= 1,
                    Dir::Down => curr.1 += 1,
                    Dir::Left => curr.0 -= 1,
                    Dir::Right => curr.0 += 1
                };

                println!("{:?} {:?}", step, curr);
                map.0[curr.1 as usize][curr.0 as usize] = Cell::Trench;
            });
        });

        map
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Terrain,
    Trench,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Terrain => ".",
            Cell::Trench => "#"
        })
    }
}

struct Map(Vec<Vec<Cell>>);

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter()
            .map(|row|
                row.iter().map(|cell| cell.to_string()).join(""))
            .join("\n")
        )
    }
}

impl Map {
    fn fill(&mut self) -> &Self {
        (0..self.0.len())
            .for_each(|y| {
                // find start, end
                let start =
                    (0..self.0[0].len())
                        .find(|x| self.0[y][*x] == Cell::Trench);

                if let Some(start) = start {
                    let end = ((start + 1)..=(self.0[0].len() - 1)).rev()
                        .find(|x| self.0[y][*x] == Cell::Trench);

                    if let Some(end) = end {
                        ((start + 1)..end).for_each(|x| {
                            self.0[y][x] = Cell::Trench
                        })
                    } else {
                        panic!("incomplete range not expected on row {} starting on {}", y, start)
                    }
                } else {
                    // expect blank line
                }
            });

        self
    }

    fn area(&self) -> u32 {
        self.0.iter()
            .map(|row|
                row.iter().filter(|cell| **cell == Cell::Trench).count() as u32
            )
            .sum()
    }
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid direction {}", value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day18::{Plan, run};

    const INPUT_PLAN: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    const PARTIAL_TRENCH_OUTLINE: &str = r#"#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######"#;

    #[test]
    fn test_1_outline() {
        assert_eq!(Plan::from(INPUT_PLAN).outline_trench().to_string(), PARTIAL_TRENCH_OUTLINE);
    }


    const PARTIAL_TRENCH_FILL: &str = r#"#######
#######
#######
..#####
..#####
#######
#####..
#######
.######
.######"#;

    #[test]
    fn test_1_fill() {
        assert_eq!(Plan::from(INPUT_PLAN).outline_trench().fill().to_string(), PARTIAL_TRENCH_FILL);
    }

    #[test]
    fn test_1() {
        assert_eq!(Plan::from(INPUT_PLAN).outline_trench().fill().area(), 62);
    }
}