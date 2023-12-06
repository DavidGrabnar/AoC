use std::cell::RefCell;
use std::collections::HashSet;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub fn run(input: &str) -> i32 {
    let grid = Grid::new(input);
    // println!("{:?}", grid);
    // println!("{:?}", grid.adjacent());
    return grid.adjacent()
        .iter().map(|v| *v)
        .sum::<u32>() as i32;
}

pub fn run2(input: &str) -> i32 {
    let grid = Grid::new(input);
    println!("{:?}", grid);
    println!("{:?}", grid.gears());
    return grid.gears()
        .iter().map(|v| *v)
        .sum::<u32>() as i32;
}

#[derive(Debug)]
struct WRc(Rc<RefCell<u32>>);

impl PartialEq for WRc {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for WRc {}

impl Hash for WRc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}


#[derive(Debug)]
enum Cell {
    Number(WRc),
    Gear,
    Symbol,
    None,
}

#[derive(Debug)]
struct Grid(Vec<Vec<Cell>>);

impl Grid {
    fn new(input: &str) -> Self {
        Grid(
            input.lines()
                .map(|line| {
                    let mut ongoing = Rc::new(RefCell::new(0u32));
                    line.chars()
                        .map(|c| match c {
                            raw @ '0'..='9' => {
                                let curr = raw.to_digit(10).expect("not a digit");
                                let prev = ongoing.borrow().clone();
                                *ongoing.borrow_mut() = prev * 10 + curr;
                                Cell::Number(WRc(ongoing.clone()))
                            },
                            '*' => {
                                ongoing = Rc::new(RefCell::new(0u32));
                                Cell::Gear
                            },
                            '.' => {
                                ongoing = Rc::new(RefCell::new(0u32));
                                Cell::None
                            },
                            _ => {
                                ongoing = Rc::new(RefCell::new(0u32));
                                Cell::Symbol
                            }
                        })
                        .collect()
                })
                .collect::<Vec<_>>()
        )
    }

    fn adjacent(&self) -> Vec<u32> {
        let mut out = HashSet::new();
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Number(val) = cell {
                    if out.contains(val) {
                        // if number already considered, skip
                        continue;
                    }
                    let x = x as isize;
                    let y = y as isize;
                    let neighbours = [
                        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                        (x - 1, y    ), (x, y    ), (x + 1, y    ),
                        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
                    ];
                    for (nx, ny) in neighbours {
                        match self.0
                            .get(ny as usize).unwrap_or(&vec![])
                            .get(nx as usize).unwrap_or(&Cell::None) {
                            Cell::Symbol|Cell::Gear => {
                                // if neighbour is symbol, add to set and skip
                                out.insert(val);
                                continue;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        return out.iter().map(|val| val.0.borrow().clone()).collect();
    }

    fn gears(&self) -> Vec<u32> {
        let mut out = vec![];
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Gear = cell {
                    let x = x as isize;
                    let y = y as isize;
                    let neighbours = [
                        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                        (x - 1, y    ), (x, y    ), (x + 1, y    ),
                        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
                    ];
                    let mut nums = HashSet::new();
                    let blank =  vec![];
                    for (nx, ny) in neighbours {
                        match self.0
                            .get(ny as usize).unwrap_or(&blank)
                            .get(nx as usize).unwrap_or(&Cell::None) {
                            Cell::Number(val) => {
                                // if neighbour is symbol, add to set and skip
                                nums.insert(val);
                                continue;
                            }
                            _ => {}
                        }
                    }
                    if nums.len() == 2 {
                        out.push(nums.iter().fold(1, |acc, el| acc * *el.0.borrow()));
                    }
                }
            }
        }

        return out;
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day03::{run, run2};

    const INPUT_1: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 4361);
    }

    #[test]
    fn test_2() {
        assert_eq!(run2(INPUT_1), 467835);
    }
}