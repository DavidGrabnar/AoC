use std::fs::File;
use std::{fmt, io};
use std::fmt::Formatter;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "input.txt";

enum Direction {
    Forward,
    Up,
    Down
}

struct Movement {
    direction: Direction,
    value: i32
}

impl Movement {
    fn new<>(raw: String<>) -> Result<Movement, String> {
        let mut split = raw.split_whitespace();
        let (raw_direction, raw_value) = (split.next().ok_or("No direction")?, split.next().ok_or("No value")?);

        let direction = Movement::parse_direction(raw_direction)?;
        let value = raw_value.parse::<i32>().map_err(|_| "Failed to parse value")?;

        Ok(Movement { direction, value })
    }

    fn parse_direction(raw: &str) -> Result<Direction, String> {
        match raw {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(format!("Invalid direction '{}'", raw))
        }
    }
}

struct Position(i32, i32, i32);

impl Position {
    fn apply(&mut self, movement: Movement) -> &mut Position {
        match movement.direction {
            Direction::Forward => {
                self.0 += movement.value;
                self.1 += self.2 * movement.value;
            },
            Direction::Up => self.2 -= movement.value,
            Direction::Down => self.2 += movement.value
        }
        self
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

fn main() -> io::Result<()> {
    let mut initial_position = Position(0, 0, 0);

    let final_position = BufReader::new(File::open(INPUT_FILE)?)
        .lines()
        .map(|raw| Movement::new(raw.unwrap()).unwrap())
        .fold(
            &mut initial_position,
            |position, movement| position.apply(movement)
        );

    println!("Final position: {}, result {}", final_position, final_position.0 * final_position.1);

    Ok(())
}
