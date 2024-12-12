use std::collections::HashMap;

pub fn run(input: &str) -> u64 {
    let (workflows, parts, _) = input.split_whitespace()
        .fold((vec![], vec![], false), |(mut workflows, mut parts, mut on_parts), row| {
            if !on_parts && row.starts_with("{") {
                on_parts = true
            }
            if on_parts {
                parts.push(row)
            } else {
                workflows.push(row)
            }

            (workflows, parts, on_parts)
        });


    let workflows = Workflows::from(workflows.join("\n").as_str());
    let parts: Vec<Part> = parts.into_iter().map(|el| el.into()).collect();

    let accepted = parts.iter().filter(|part| {
        let mut curr_name = "in";
        let curr_rule = workflows.0.get(curr_name).unwrap();

    });

    0
}

#[derive(Debug)]
enum Type {
    X,
    M,
    A,
    S
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("invalid type {}", value)
        }
    }
}

#[derive(Debug)]
struct Part([u64; 4]);

impl From<&str> for Part {
    // assumes correct order
    fn from(value: &str) -> Self {
        Self(
            value.replace("{", "").replace("}","").split(",")
                .map(|el| {
                    let mut split = el.split("=");
                    split.next();
                    let next = split.next();
                    next.unwrap().parse::<u64>().unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        )
    }
}

impl Part {
    fn value(&self, t: &Type) -> u64 {
        match t {
            Type::X => self.0[0],
            Type::M => self.0[1],
            Type::A => self.0[2],
            Type::S => self.0[3],
        }
    }
}

#[derive(Debug)]
enum Limit {
    Lt,
    Gt
}

impl From<char> for Limit {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Lt,
            '>' => Self::Gt,
            _ => panic!("invalid limit {}", value)
        }
    }
}

#[derive(Debug)]
struct Cmp(Type, Limit, u64);

impl From<&str> for Cmp {
    fn from(value: &str) -> Self {
        let (delim, limit) = if value.contains(">") {
            (">", Limit::Gt)
        } else {
            ("<", Limit::Lt)
        };

        let mut split = value.split(delim);

        Self (
            split.next().unwrap().into(),
            limit,
            split.next().unwrap().parse().unwrap()
        )
    }
}

#[derive(Debug)]
enum Rule {
    Cmp(Cmp, String),
    GoTo(String)
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if !value.contains(":") {
            Self::GoTo(value.to_string())
        } else {
            let mut split = value.split(":");
            Self::Cmp (
                split.next().unwrap().into(),
                split.next().unwrap().to_string()
            )
        }
    }
}

#[derive(Debug)]
struct Workflows(HashMap<String, Vec<Rule>>);

impl From<&str> for Workflows {
    fn from(value: &str) -> Self {
        Self (
            value.lines().map(
                |row| {
                    let mut split = row.split("{");

                    (
                        split.next().unwrap().to_string(),
                        split.next().unwrap()
                            .replace("}", "")
                            .split(",")
                            .map(|el| el.into())
                            .collect()
                    )
                }
            )
                .collect()
        )
    }
}

impl Workflows {
    fn process(&self, part: &Part) -> String {
        let mut workflow = "in";
        if let Some(cmd) = self.0.get(workflow) {
            cmd.
        } else {
            workflow
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day19::{Part, run, Workflows};

    const INPUT_1: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn test_1() {
        assert_eq!(run(INPUT_1), 142);
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