use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;

pub fn run(input: &str) -> i32 {
    let r = input.lines()
        .map(|line| Bid::new(line))
        .sorted()
        .enumerate()
        .collect::<Vec<_>>();

    // r.iter().for_each(|(idx, bid)| {
    //     println!("{} {} - {:?} - {:?}", idx + 1, bid.1, Value::from(bid.0), bid.0);
    // });

    r.iter().map(|(idx, bid)| (*idx as u32 + 1) * bid.1)
        .sum::<u32>() as i32
}

pub fn run2(input: &str) -> i32 {
    let r = input.lines()
        .map(|line| Bid::new(line))
        .sorted()
        .enumerate()
        .collect::<Vec<_>>();

    // r.iter().for_each(|(idx, bid)| {
    //     println!("{} {} - {:?} - {:?}", idx + 1, bid.1, Value::from(bid.0), bid.0);
    // });

    r.iter().map(|(idx, bid)| (*idx as u32 + 1) * bid.1)
        .sum::<u32>() as i32
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, Ord, PartialOrd)]
enum Card {
    Jack, // To run part 2, move Jack to here
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    // Jack, // To run part 1, move Jack to here
    Queen,
    King,
    Ace
}

impl Card {
    fn new(raw: char) -> Self {
        match raw {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("invalid card value: {}", raw)
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Type {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(Debug)]
struct Value(Type, u32);

impl Value {
    fn value(&self) -> u64 {
        (self.0 as u64 * 10_000_000_000) + self.1 as u64
    }
}

impl From<Hand> for Value {
    fn from(value: Hand) -> Self {
        let part_2 = true; // change to false for part 1

        let mut map: HashMap<Card, u32> = HashMap::new();
        for el in value.0 {
            map.insert(el, map.get(&el).cloned().unwrap_or(0) + 1);
        }

        let num_jokers = map.get(&Card::Jack).cloned().unwrap_or(0);
        if part_2 {
            // remove Jack, so it's not considered for type
            map.remove(&Card::Jack);
        }

        let part_1_type = if map.iter().filter(|(c, v)| **v == 5).next().is_some() {
            Type::FiveOfAKind
        } else if map.iter().filter(|(c, v)| **v == 4).next().is_some() {
            Type::FourOfAKind
        } else if map.iter().filter(|(c, v)| **v == 3).next().is_some() {
            if map.iter().filter(|(c, v)| **v == 2).next().is_some() {
                Type::FullHouse
            } else {
                Type::ThreeOfAKind
            }
        } else {
            let pairs = map.iter().filter(|(c, v)| **v == 2).map(|(c, v)| c).collect::<Vec<_>>();
            if pairs.len() == 2 {
                Type::TwoPairs
            } else if pairs.len() == 1 {
                Type::OnePair
            } else {
                Type::HighCard
            }
        };

        let typ = if part_2 {
            let x = match part_1_type {
                Type::HighCard => if num_jokers == 5 || num_jokers == 4 {
                    Type::FiveOfAKind
                } else if num_jokers == 3 {
                    Type::FourOfAKind
                } else if num_jokers == 2 {
                    Type::ThreeOfAKind
                } else if num_jokers == 1 {
                    Type::OnePair
                } else {
                    part_1_type
                }
                Type::OnePair => if num_jokers == 3 {
                    Type::FiveOfAKind
                } else if num_jokers == 2 {
                    Type::FourOfAKind
                } else if num_jokers == 1 {
                    Type::ThreeOfAKind
                } else {
                    part_1_type
                }
                Type::TwoPairs => if num_jokers == 1 {
                    Type::FullHouse
                } else {
                    part_1_type
                }
                Type::ThreeOfAKind => if num_jokers == 2 {
                    Type::FiveOfAKind
                } else if num_jokers == 1 {
                    Type::FourOfAKind
                } else {
                    part_1_type
                }
                Type::FullHouse => part_1_type,
                Type::FourOfAKind => if num_jokers == 1 {
                    Type::FiveOfAKind
                } else {
                    part_1_type
                }
                Type::FiveOfAKind => part_1_type
            };

            println!("Conv {:?} -> {:?} - {} - {:?}", part_1_type, x, num_jokers, map);
            x
        } else {
            part_1_type
        };

        Value(
            typ,
            value.value()
        )
    }
}

#[derive(Debug, Copy, Clone)]
struct Hand([Card; 5]);

impl Hand {
    fn new(raw: &str) -> Self {
        Hand(
            raw.chars()
                .map(|c| Card::new(c)).collect::<Vec<_>>()
                .try_into()
                .unwrap()
        )
    }

    fn value(&self) -> u32 {
        self.0.iter().rev()
            .enumerate().map(|(idx, card) | 10u32.pow(idx as u32 * 2) * *card as u32)
            .sum()
    }
}

#[derive(Debug)]
struct Bid(Hand, u32, Value);

impl Bid {
    fn new(raw: &str) -> Self {
        let mut split = raw.split_whitespace();

        let hand = Hand::new(split.next().unwrap());
        Bid(
            hand,
            split.next().unwrap().parse().unwrap(),
            Value::from(hand)
        )
    }

    fn weight(&self) -> u64 {
        self.2.value()
    }
}

impl Eq for Bid {}

impl PartialEq<Self> for Bid {
    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

impl PartialOrd<Self> for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day07::{run, run2};

    const INPUT_1: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 6440);
    }

    #[test]
    fn test_2() {
        assert_eq!(run2(INPUT_1), 5905);
    }
}