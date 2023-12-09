use std::collections::{BTreeMap};
use std::hash::{Hash};
use std::sync::Arc;
use std::thread;
use regex::{Regex};
use itertools::Itertools;

pub fn run(input: &str) -> i32 {
    let seeds = Seeds::new(input);
    println!("{:?}", seeds.0);
    let mapper = Mapper::new(input);
    println!("{:?}", mapper);

    seeds.0.iter().map(|s| mapper.resolve_seed(s))
        .map(|l| l.0)
        .min().expect("no seeds to resolve") as i32
}

pub fn run2(input: &str) -> i32 {
    let seeds = SeedRanges::new(input);
    println!("{:?}", seeds.0);
    let mapper = Mapper::new(input);
    println!("{:?}", mapper);

    let mapper_ref = Arc::new(mapper);
    seeds.0.iter()
        .map(|(start, len)| {
            let start = *start;
            let len = *len;
            let mapper_ref = Arc::clone(&mapper_ref);
            thread::spawn(move || {
                println!("spawned thread for: {} {}", start, len);
                let res = ((start..(start + len))
                    .map(Seed::from)
                    .map(|s| mapper_ref.resolve_seed(&s))
                    .map(|l| l.0)
                    .min()
                    .unwrap()
                );
                println!("finished thread with min res: {}", res);
                res
            })
        })
        .collect::<Vec<_>>() // force lazy iter to run
        .into_iter()
        .map(|h| h.join().unwrap())
        .min().expect("no seeds to resolve") as i32
}

macro_rules! make_type {
    ($name:ident) => {
        #[derive(Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone)]
        struct $name(u32);

        impl From<u32> for $name {
            fn from(value: u32) -> Self {
                $name(value)
            }
        }

        impl ToU32 for $name {
            fn to(&self) -> u32 {
                self.0
            }
        }
    };
}

make_type!(Seed);
make_type!(Soil);
make_type!(Fertilizer);
make_type!(Water);
make_type!(Light);
make_type!(Temperature);
make_type!(Humidity);
make_type!(Location);

struct Range(u32, u32, u32);

impl Range {
    fn new(input: &str) -> Self {
        let mut split = input.split(" ");
        Self (
            split.next().unwrap().parse::<u32>().unwrap(),
            split.next().unwrap().parse::<u32>().unwrap(),
            split.next().unwrap().parse::<u32>().unwrap(),
        )
    }
}

trait ToU32 {
    fn to(&self) -> u32;
}

// DISCLAIMER: THIS DOES NOT WORK FOR OVERLAPPING RANGES OR RANGES WITH SAME START
#[derive(Debug, Default)]
struct RangeMap<K: Eq + PartialEq + Hash + PartialOrd + Ord + Clone + ToU32, V: ToU32> {
    values: BTreeMap<K, (V, u32/*, u32*/)>, // (value, length, order)
}

impl<K: Eq + PartialEq + Hash + PartialOrd + Ord + Clone + ToU32, V: ToU32 + From<u32>> RangeMap<K, V> {
    fn insert(&mut self, key: K, len: u32, value: V) {
        self.values.insert(key.clone(), (value, len - 1));
    }

    fn get(&self, key: &K) -> Option<V> {
        match self.values.iter()
            .filter(|(k, (_, len))| {
                *k <= key
                    && k.to() as u64 + *len as u64 >= key.to() as u64
            })
            .map(|(k, _)| k)
            .min() {
            None => return None,
            Some(k) => {
                match self.values.get(k) {
                    None => panic!("Key not found in map"),
                    Some((val, _)) => {
                        let diff = key.to() - k.to();
                        return Some((val.to() + diff).into())
                    }
                }
            }
        }
    }
}

impl<K: Eq + PartialEq + Hash + PartialOrd + Ord + Clone + ToU32 + From<u32>, V: ToU32 + From<u32>> RangeMap<K, V> {
    fn fill(&mut self, ranges: Vec<Range>) {
        for range in ranges {
            self.insert(range.1.into(), range.2, range.0.into());
        }
    }
}

struct Seeds(Vec<Seed>);

impl Seeds {
    fn new(input: &str) -> Self {
        Seeds(
            Regex::new(r"seeds: ((\d+ ?)+)")
                .expect("invalid regex pattern")
                .captures(input)
                .expect("missing seeds data")
                .extract::<2>()
                .1[0].split_whitespace()
                .map(|v| Seed(v.parse().expect("invalid seed value")))
                .collect()
        )
    }
}

struct SeedRanges(Vec<(u32, u32)>);

impl SeedRanges {
    fn new(input: &str) -> Self {
        SeedRanges(
            Regex::new(r"seeds: ((\d+ ?)+)")
                .expect("invalid regex pattern")
                .captures(input)
                .expect("missing seeds data")
                .extract::<2>()
                .1[0].split_whitespace()
                .map(|el| el.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .chunks(2)
                .map(|c| (c[0], c[1]))
                .collect::<Vec<_>>()
        )
    }
}

#[derive(Debug, Default)]
struct Mapper {
    seed_to_soil: RangeMap<Seed, Soil>,
    soil_to_fertilizer: RangeMap<Soil, Fertilizer>,
    fertilizer_to_water: RangeMap<Fertilizer, Water>,
    water_to_light: RangeMap<Water, Light>,
    light_to_temperature: RangeMap<Light, Temperature>,
    temperature_to_humidity: RangeMap<Temperature, Humidity>,
    humidity_to_location: RangeMap<Humidity, Location>,
}

impl Mapper {
    fn new(input: &str) -> Self {
        Regex::new(r"(seed|soil|fertilizer|water|light|temperature|humidity)-to-[a-z]+ map:[\n\r]*((\d+ \d+ \d+[\n\r]*)+)")
            .expect("invalid regex pattern")
            .captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [v1, v2, _])|  (v1, v2.trim().lines().map(str::trim).map(Range::new).collect::<Vec<Range>>()))
            .fold(Self::default(), |mut out, (key, values)| {
                // println!("match {}", key);
                match key {
                    "seed" => out.seed_to_soil.fill(values),
                    "soil" => out.soil_to_fertilizer.fill(values),
                    "fertilizer" => out.fertilizer_to_water.fill(values),
                    "water" => out.water_to_light.fill(values),
                    "light" => out.light_to_temperature.fill(values),
                    "temperature" => out.temperature_to_humidity.fill(values),
                    "humidity" => out.humidity_to_location.fill(values),
                    _ => panic!("invalid key {}", key),
                }
                return out;
            })
    }

    fn resolve_seed(&self, seed: &Seed) -> Location {
        let v = self.seed_to_soil.get(seed).unwrap_or(seed.0.into());
        // print!("{:?} -> {:?}", seed, v);
        let v = self.soil_to_fertilizer.get(&v).unwrap_or(v.0.into());
        // print!("-> {:?}", v);
        let v = self.fertilizer_to_water.get(&v).unwrap_or(v.0.into());
        // print!("-> {:?}", v);
        let v = self.water_to_light.get(&v).unwrap_or(v.0.into());
        // print!("-> {:?}", v);
        let v = self.light_to_temperature.get(&v).unwrap_or(v.0.into());
        // print!("-> {:?}", v);
        let v = self.temperature_to_humidity.get(&v).unwrap_or(v.0.into());
        // print!("-> {:?}", v);
        let v = self.humidity_to_location.get(&v).unwrap_or(v.0.into());
        // println!("-> {:?}", v);
        v
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day05::{run, run2};

    const INPUT_1: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 35);
    }

    #[test]
    fn test_2() {
        assert_eq!(run2(INPUT_1), 46);
    }
}