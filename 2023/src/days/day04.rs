use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::hash::{Hash, Hasher};

pub fn run(input: &str) -> i32 {
    let cards = Cards::new(input);
    return cards.value() as i32;
}

pub fn run2(input: &str) -> i32 {
    let cards = Cards::new(input);
    return cards.collected_total() as i32;
}

#[derive(Debug)]
struct Cards(BTreeMap<u32, Card>);

impl Cards {
    fn new(input: &str) -> Self {
        Cards(
            input.lines()
                .map(Card::new)
                .map(|c| (c.0, c))
                .collect()
        )
    }

    fn value(&self) -> u32 {
        self.0.values().map(Card::value).sum()
    }

    fn collected_total(&self) -> u32 {
        let mut amounts = self.0.keys().map(|k| (*k, 1)).collect::<HashMap<u32, u32>>();

        self.0.iter().for_each(|(k, v)| {
            let amount = *amounts.get(k).expect("card id not in amounts map");
            v.collected().iter().for_each(|collected| {
                let current = amounts.get(collected).expect("card id not in amounts map");
                amounts.insert(*collected, current + amount);
            });
        });
        return amounts.values().sum();
    }
}

#[derive(Debug)]
struct Card(u32, HashSet<u32>, HashSet<u32>);

impl Card {
    fn new(raw: &str) -> Self {
        let mut split = raw.split(": ");
        let mut label = split.next().expect("not enough items after ': ' split")
            .split_whitespace();

        label.next();
        let id = label.next().expect("not enough items after ' ' split")
            .parse::<u32>().expect("not an integer");

        let mut split = split.next().expect("not enough items after ': ' split")
            .split("| ");

        Card(
            id,
            split.next().expect("not enough items after '| ' split")
                .split_whitespace().flat_map(str::parse::<u32>).collect(),
            split.next().expect("not enough items after '| ' split")
                .split_whitespace().flat_map(str::parse::<u32>).collect()
        )
    }

    fn matches(&self) -> Vec<u32> {
        self.1.intersection(&self.2).cloned().collect()
    }

    fn value(&self) -> u32 {
        let len = self.matches().len();
        if len == 0 {
            return 0;
        }
        2u32.pow((len - 1) as u32)
    }

    fn collected(&self) -> Vec<u32> {
        (0..self.matches().len()).map(|idx| self.0 + idx as u32 + 1).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day04::{run, run2};

    const INPUT_1: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 13);
    }

    #[test]
    fn test_2() {
        assert_eq!(run2(INPUT_1), 30);
    }
}