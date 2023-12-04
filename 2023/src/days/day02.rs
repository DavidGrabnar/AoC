use std::error::Error;

pub fn run(input: &str) -> i32 {
    let games = Games::new(input);
    println!("{:?}", games);
    return 0;
}

#[derive(Debug)]
struct Games(Vec<Game>);

impl Games {
    fn new(raw: &str) -> Self {
        Games(
            raw.split("\n")
                .map(|el| Game::new(el))
                .collect::<Vec<_>>()
        )
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
                .collect::<Vec<_>>()
        )
    }
}

#[derive(Debug)]
struct Subset(Vec<Cubes>);

impl Subset {
    fn new(raw: &str) -> Self {
        Self(raw.split(", ")
            .map(|el| Cubes::new(el))
            .collect::<Vec<_>>()
        )
    }
}

#[derive(Debug)]
struct Cubes(i32, Cube);

impl Cubes {
    fn new(raw: &str) -> Self {
        let mut split = raw.split(" ");

        Cubes(
            split.next().expect("not enough elements after split")
                .parse::<i32>().expect("count not an integer"),
            Cube::new(
                split.next().expect("not enough elements after split")),
        )
    }
}

#[derive(Debug)]
enum Cube {
    RED,
    BLUE,
    GREEN
}

impl Cube {
    fn new(raw: &str) -> Self {
        match raw {
            "red" => Self::RED,
            "blue" => Self::BLUE,
            "green" => Self::GREEN,
            _ => panic!("No color named {}", raw)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day02::run;

    const INPUT_1: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 8);
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
        assert_eq!(run(INPUT_2), 281);
    }
}