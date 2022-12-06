extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "input.txt";

/*
A - Rock
B - Paper
C - Scissors

X - Rock
Y - Paper
Z - Scissors

Rock - 1
Paper - 2
Scissors - 3

Lost - 0
Draw - 3
Win - 6
 */

fn main() {
    let first_score = BufReader::new(File::open(INPUT_FILE).unwrap())
        .lines()
        .fold(0,
              |score, raw| -> i32 {
                  let raw_val = raw.unwrap();
                  let player = match raw_val.chars().nth(2).unwrap() {
                      'X' => 1,
                      'Y' => 2,
                      'Z' => 3,
                      invalid => panic!("Third character must be [X, Y, Z], got '{}'", invalid)
                  };

                  let round = match raw_val.chars().nth(0).unwrap() {
                      'A' => match player {
                          1 => 3,
                          2 => 6,
                          3 => 0,
                          _ => panic!("Player should be [1, 2, 3], got '{}'", player)
                      },
                      'B' => match player {
                          1 => 0,
                          2 => 3,
                          3 => 6,
                          _ => panic!("Player should be [1, 2, 3], got '{}'", player)
                      },
                      'C' => match player {
                          1 => 6,
                          2 => 0,
                          3 => 3,
                          _ => panic!("Player should be [1, 2, 3], got '{}'", player)
                      },
                      invalid => panic!("First character must be [A, B, C], got '{}'", invalid)
                  };

                  println!("{}: {} ({} + {})", raw_val, player + round, player, round);
                  score + player + round
              });

    println!("Part 1 result: {}", first_score);

    let second_score = BufReader::new(File::open(INPUT_FILE).unwrap())
        .lines()
        .fold(0,
              |score, raw| -> i32 {
                  let raw_val = raw.unwrap();
                  let round = match raw_val.chars().nth(2).unwrap() {
                      'X' => 0,
                      'Y' => 3,
                      'Z' => 6,
                      invalid => panic!("Third character must be [X, Y, Z], got '{}'", invalid)
                  };

                  let player = match raw_val.chars().nth(0).unwrap() {
                      'A' => match round {
                          0 => 3,
                          3 => 1,
                          6 => 2,
                          _ => panic!("Round should be [0, 3, 6], got '{}'", round)
                      },
                      'B' => match round {
                          0 => 1,
                          3 => 2,
                          6 => 3,
                          _ => panic!("Round should be [0, 3, 6], got '{}'", round)
                      },
                      'C' => match round {
                          0 => 2,
                          3 => 3,
                          6 => 1,
                          _ => panic!("Round should be [0, 3, 6], got '{}'", round)
                      },
                      invalid => panic!("First character must be [A, B, C], got '{}'", invalid)
                  };

                  println!("{}: {} ({} + {})", raw_val, player + round, player, round);
                  score + player + round
              });

    println!("Part 2 result: {}", second_score);
}
