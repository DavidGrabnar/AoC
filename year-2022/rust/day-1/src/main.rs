use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "input.txt";

fn main() {
    let mut x = BufReader::new(File::open(INPUT_FILE).unwrap())
        .lines()
        .fold((vec![], 0),
              |(mut list, mut curr), raw| -> (Vec<i32>, i32) {
                let raw_val = raw.unwrap();
                if raw_val.len() == 0 {
                    list.push(curr);
                    (list, 0)
                } else {
                    curr += raw_val.parse::<i32>().unwrap();
                    (list, curr)
                }
            });

    x.0.push(x.1);

    println!("Part 1 result: {:?}\n Details: {:?}", x.0.iter().max(), x.0);

    let mut max: [i32; 3] = [0, 0, 0];
    x.0.iter()
        .for_each(|val| {
            if val > &max[0] {
                max[2] = max[1];
                max[1] = max[0];
                max[0] = *val;
            } else if val > &max[1] {
                max[2] = max[1];
                max[1] = *val;
            } else if val > &max[2] {
                max[2] = *val;
            }
        });

    println!("Part 2 result: {:?}\n Details: {:?}", max.iter().sum::<i32>(), max);
}
