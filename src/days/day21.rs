use std::collections::HashMap;

use crate::days;

pub enum Monkey {
    Eq(String, String),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Num(i64),
}

impl Monkey {
    fn get_num(&self, input: &HashMap<String, Monkey>) -> i64 {
        match self {
            Monkey::Eq(_, _) => unreachable!(),
            Monkey::Add(a, b) => input[a].get_num(input) + input[b].get_num(input),
            Monkey::Sub(a, b) => input[a].get_num(input) - input[b].get_num(input),
            Monkey::Mul(a, b) => input[a].get_num(input) * input[b].get_num(input),
            Monkey::Div(a, b) => input[a].get_num(input) / input[b].get_num(input),
            Monkey::Num(n) => *n,
        }
    }

    fn find(&self, you: &String, to_find: &String, input: &HashMap<String, Monkey>) -> bool {
        if you == to_find {
            return true;
        }
        match self {
            Monkey::Eq(a, b) => {
                if a == to_find || b == to_find {
                    true
                } else {
                    input[a].find(a, to_find, input) || input[b].find(b, to_find, input)
                }
            }
            Monkey::Add(a, b) => {
                if a == to_find || b == to_find {
                    true
                } else {
                    input[a].find(a, to_find, input) || input[b].find(b, to_find, input)
                }
            }
            Monkey::Sub(a, b) => {
                if a == to_find || b == to_find {
                    true
                } else {
                    input[a].find(a, to_find, input) || input[b].find(b, to_find, input)
                }
            }
            Monkey::Mul(a, b) => {
                if a == to_find || b == to_find {
                    true
                } else {
                    input[a].find(a, to_find, input) || input[b].find(b, to_find, input)
                }
            }
            Monkey::Div(a, b) => {
                if a == to_find || b == to_find {
                    true
                } else {
                    input[a].find(a, to_find, input) || input[b].find(b, to_find, input)
                }
            }
            Monkey::Num(_) => false,
        }
    }

    fn left_side(&self) -> String {
        match self {
            Monkey::Eq(a, _) => a.to_string(),
            Monkey::Add(a, _) => a.to_string(),
            Monkey::Sub(a, _) => a.to_string(),
            Monkey::Mul(a, _) => a.to_string(),
            Monkey::Div(a, _) => a.to_string(),
            Monkey::Num(_) => unreachable!(),
        }
    }

    fn right_side(&self) -> String {
        match self {
            Monkey::Eq(_, b) => b.to_string(),
            Monkey::Add(_, b) => b.to_string(),
            Monkey::Sub(_, b) => b.to_string(),
            Monkey::Mul(_, b) => b.to_string(),
            Monkey::Div(_, b) => b.to_string(),
            Monkey::Num(_) => unreachable!(),
        }
    }

    fn match_num(&self, you: String, num: i64, input: &HashMap<String, Monkey>) -> i64 {
        if you == "humn" {
            return num;
        }

        match self {
            Monkey::Eq(_, _) => unreachable!(),
            Monkey::Add(a, b) => {
                let left = input[a].find(a, &"humn".to_string(), input);
                let right = input[b].find(b, &"humn".to_string(), input);
                if !left && right {
                    input[b].match_num(b.clone(), num - input[a].get_num(input), input)
                } else if left && !right {
                    input[a].match_num(a.clone(), num - input[b].get_num(input), input)
                } else {
                    panic!("Da hell")
                }
            }
            Monkey::Sub(a, b) => {
                let left = input[a].find(a, &"humn".to_string(), input);
                let right = input[b].find(b, &"humn".to_string(), input);
                if !left && right {
                    input[b].match_num(b.clone(), input[a].get_num(input) - num, input)
                } else if left && !right {
                    input[a].match_num(a.clone(), input[b].get_num(input) + num, input)
                } else {
                    panic!("Da hell")
                }
            }
            Monkey::Mul(a, b) => {
                let left = input[a].find(a, &"humn".to_string(), input);
                let right = input[b].find(b, &"humn".to_string(), input);
                if !left && right {
                    input[b].match_num(b.clone(), num / input[a].get_num(input), input)
                } else if left && !right {
                    input[a].match_num(a.clone(), num / input[b].get_num(input), input)
                } else {
                    panic!("Da hell")
                }
            }
            Monkey::Div(a, b) => {
                let left = input[a].find(a, &"humn".to_string(), input);
                let right = input[b].find(b, &"humn".to_string(), input);
                if !left && right {
                    input[b].match_num(b.clone(), input[a].get_num(input) / num, input)
                } else if left && !right {
                    input[a].match_num(a.clone(), input[b].get_num(input) * num, input)
                } else {
                    panic!("Da hell")
                }
            }
            Monkey::Num(_) => unreachable!(),
        }
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = HashMap<String, Monkey>;

    fn get_num(&self) -> u8 {
        21
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        (input["root"].get_num(input).to_string(), true)
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        let input = input;
        let root = Monkey::Eq(input["root"].left_side(), input["root"].right_side());

        let left = input[&root.left_side()].find(&"root".to_string(), &"humn".to_string(), &input);
        let right = input[&root.right_side()].find(&"root".to_string(), &"humn".to_string(), &input);

        if !left && right {
            (
                input[&root.right_side()]
                    .match_num(
                        root.right_side(),
                        input[&root.left_side()].get_num(&input),
                        &input,
                    )
                    .to_string(),
                true,
            )
        } else if left && !right {
            (
                input[&root.left_side()]
                    .match_num(
                        root.left_side(),
                        input[&root.right_side()].get_num(&input),
                        &input,
                    )
                    .to_string(),
                true,
            )
        } else {
            panic!("No solution found");
        }
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        let mut map = HashMap::new();

        for line in input.lines() {
            let mut split = line.split(": ");
            let key = split.next().unwrap().to_string();
            let value = split.next().unwrap();

            let value = if value.contains(" + ") {
                let mut split = value.split(" + ");
                let a = split.next().unwrap().to_string();
                let b = split.next().unwrap().to_string();
                Monkey::Add(a, b)
            } else if value.contains(" - ") {
                let mut split = value.split(" - ");
                let a = split.next().unwrap().to_string();
                let b = split.next().unwrap().to_string();
                Monkey::Sub(a, b)
            } else if value.contains(" * ") {
                let mut split = value.split(" * ");
                let a = split.next().unwrap().to_string();
                let b = split.next().unwrap().to_string();
                Monkey::Mul(a, b)
            } else if value.contains(" / ") {
                let mut split = value.split(" / ");
                let a = split.next().unwrap().to_string();
                let b = split.next().unwrap().to_string();
                Monkey::Div(a, b)
            } else {
                Monkey::Num(value.parse().unwrap())
            };

            map.insert(key, value);
        }

        map
    }
}
