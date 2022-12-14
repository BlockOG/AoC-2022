use crate::days;
use itertools::Itertools;

fn sub_strings(source: &str, sub_size: usize) -> Vec<String> {
    source
        .chars()
        .chunks(sub_size)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<_>>()
}

pub struct Day {}

impl days::Day for Day {
    type Input = (String, String);

    fn get_num(&self) -> u8 {
        5
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> String {
        let crates_string = &input.0;
        let to_move = &input.1;

        let mut crates = vec![];
        for row_string in crates_string.lines() {
            if row_string.get(1..2).unwrap() == "1" {
                continue;
            }
            let row = sub_strings(row_string, 4);
            for i in 0..row.len() {
                if let None = crates.get(i) {
                    crates.push(vec![]);
                }
            }
            for (i, elem) in row.iter().enumerate() {
                if elem.get(1..2).unwrap() != " " {
                    crates
                        .get_mut(i)
                        .unwrap()
                        .push(elem.chars().nth(1).unwrap());
                }
            }
        }

        for stack in crates.iter_mut() {
            stack.reverse();
        }

        for line in to_move.lines() {
            let mut split = line.split(" ");
            split.next();
            let amount = split.next().unwrap().parse::<usize>().unwrap();
            split.next();
            let from = split.next().unwrap().parse::<usize>().unwrap();
            split.next();
            let to = split.next().unwrap().parse::<usize>().unwrap();

            for _ in 0..amount {
                let elem = crates.get_mut(from - 1).unwrap().pop().unwrap();
                crates.get_mut(to - 1).unwrap().push(elem);
            }
        }

        let mut result = String::new();
        for stack in crates.iter() {
            if let Some(crat) = stack.last() {
                result.push(*crat);
            }
        }
        result
    }

    fn part2(&mut self, input: &Self::Input) -> String {
        let crates_string = &input.0;
        let to_move = &input.1;

        let mut crates = vec![];
        for row_string in crates_string.lines() {
            if row_string.get(1..2).unwrap() == "1" {
                continue;
            }
            let row = sub_strings(row_string, 4);
            for i in 0..row.len() {
                if let None = crates.get(i) {
                    crates.push(vec![]);
                }
            }
            for (i, elem) in row.iter().enumerate() {
                if elem.get(1..2).unwrap() != " " {
                    crates
                        .get_mut(i)
                        .unwrap()
                        .push(elem.chars().nth(1).unwrap());
                }
            }
        }

        for stack in crates.iter_mut() {
            stack.reverse();
        }

        for line in to_move.lines() {
            let mut split = line.split(" ");
            split.next();
            let amount = split.next().unwrap().parse::<usize>().unwrap();
            split.next();
            let from = split.next().unwrap().parse::<usize>().unwrap();
            split.next();
            let to = split.next().unwrap().parse::<usize>().unwrap();

            let mut tmp = vec![];
            for _ in 0..amount {
                tmp.push(crates.get_mut(from - 1).unwrap().pop().unwrap());
            }
            for i in tmp.iter().rev() {
                crates.get_mut(to - 1).unwrap().push(*i);
            }
        }

        let mut result = String::new();
        for stack in crates.iter() {
            if let Some(crat) = stack.last() {
                result.push(*crat);
            }
        }
        result
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        let mut split = input.split("\n\n");
        (split.next().unwrap().to_string(), split.next().unwrap().to_string())
    }
}
