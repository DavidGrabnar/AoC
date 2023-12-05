use std::collections::HashMap;
use std::error::Error;

pub fn run(input: &str, input_1: &str) -> i32 {
    let games = Games::new(input);
    let available = Subset::new(input_1);
    return games.possible(&available)
        .iter()
        .map(|game| game.0)
        .sum();
}

pub fn run2(input: &str) -> i32 {
    let games = Games::new(input);
    return games.required()
        .iter()
        .map(|subset|
            subset.0.values()
                .cloned()
                .reduce(|res, el| res * el)
                .unwrap_or(0)
        )
        .sum();
}

#[derive(Debug)]
struct Games(Vec<Game>);

impl Games {
    fn new(raw: &str) -> Self {
        Games(
            raw.lines()
                .map(|el| Game::new(el))
                .collect::<Vec<_>>()
        )
    }

    fn possible(&self, available: &Subset) -> Vec<&Game> {
        self.0.iter().filter(|game| game.possible(available))
            .collect::<Vec<_>>()
    }

    fn required(&self) -> Vec<Subset> {
        self.0.iter().map(|game| game.required())
            .collect::<Vec<_>>()
    }
}

#[derive(Debug)]
struct Game(i32, Vec<Subset>);

impl Game {
    fn new(raw: &str) -> Self {
        let mut split = raw.split(": ");
        let mut game = split.next().expect("not enough elements after ': ' split")
            .split(" ");
        game.next();
        let id = game
            .next()
            .expect("not enough elements after ' ' split")
            .parse::<i32>().expect("id not an integer");

        Game(
            id,
            split.next().expect("not enough elements after ': ' split")
                .split("; ")
                .map(|el| Subset::new(el))
                .collect::<Vec<_>>(),
        )
    }

    // Subsets of game should have same or less cubes than available
    fn possible(&self, available: &Subset) -> bool {
        self.1.iter().all(|subset|
            subset.0.iter().all(|(key, count)|
                available.0.get(key)
                    .map(|count_available| count <= count_available)
                    .unwrap_or(false)
            )
        )
    }

    fn required(&self) -> Subset {
        self.1.iter().fold(Subset::blank(), |base, subset| {
            subset.0.iter().fold(base, |mut base_inner, (key, count)| {
                if let Some(count_req) = base_inner.0.get(key) {
                    if count_req < count {
                        base_inner.0.insert(*key, *count);
                    }
                } else {
                    base_inner.0.insert(*key, *count);
                }
                base_inner
            })
        })
    }
}

#[derive(Debug)]
struct Subset(HashMap<Cube, i32>);

impl Subset {
    fn blank() -> Self {
        Self(HashMap::new())
    }

    fn new(raw: &str) -> Self {
        Self(HashMap::from_iter(
            raw.split(", ")
                .map(|el| {
                    let mut col_split = el.split(" ");

                    let id = col_split.next().expect("not enough elements after split")
                        .parse::<i32>().expect("count not an integer");

                    (
                        Cube::new(col_split.next().expect("not enough elements after split")),
                        id
                    )
                })
        ))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Cube {
    RED,
    BLUE,
    GREEN,
}

impl Cube {
    fn new(raw: &str) -> Self {
        match raw {
            "red" => Self::RED,
            "blue" => Self::BLUE,
            "green" => Self::GREEN,
            _ => panic!("No color named '{}'", raw)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day02::{run, run2};

    const INPUT_1: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    const INPUT_1_1: &str = r#"12 red, 13 green, 14 blue"#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1, INPUT_1_1), 8);
    }

    const INPUT_2: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_2() {
        assert_eq!(run2(INPUT_2), 2286);
    }
}