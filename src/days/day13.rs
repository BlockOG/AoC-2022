use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
};

use crate::days;

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    List(Vec<Value>),
    Number(i64),
}

impl Value {
    fn parsing(input: &mut impl Iterator<Item = char>) -> Self {
        let mut list = vec![];
        let mut num = -1;
        loop {
            match input.next() {
                Some('[') => list.push(Value::parsing(input)),
                Some(']') => break,
                Some(',') => {
                    if num != -1 {
                        list.push(Value::Number(num));
                        num = -1;
                    }
                }
                Some(c) => {
                    if num == -1 {
                        num = 0;
                    }
                    num *= 10;
                    num += c.to_digit(10).unwrap() as i64;
                },
                None => break,
            }
        }
        if num != -1 {
            list.push(Value::Number(num));
        }
        Value::List(list)
    }

    fn parse(input: &str) -> Self {
        let mut input = input.chars();
        input.next(); // Skip the first [
        Value::parsing(&mut input)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::List(list) => {
                write!(f, "[")?;
                for (i, item) in list.iter().enumerate() {
                    if i != 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Number(num) => write!(f, "{}", num),
        }
    }
}

impl Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();
                loop {
                    match (a.next(), b.next()) {
                        (None, Some(_)) => {
                            return Ordering::Less;
                        }
                        (Some(_), None) => {
                            return Ordering::Greater;
                        }

                        (Some(a), Some(b)) => match a.cmp(b) {
                            Ordering::Equal => continue,
                            order => return order,
                        },

                        (None, None) => return Ordering::Equal,
                    }
                }
            }

            // If one is a list and the other is a number, convert the number to a list and compare
            (Value::List(_), Value::Number(a)) => {
                let convert = Value::List(vec![Value::Number(*a)]);
                self.cmp(&convert)
            }
            (Value::Number(a), Value::List(_)) => {
                let convert = Value::List(vec![Value::Number(*a)]);
                convert.cmp(other)
            }
        }
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = Vec<(Value, Value)>;

    fn get_num(&self) -> u8 {
        13
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut sum = 0;
        for (i, (a, b)) in input.iter().enumerate() {
            if a.cmp(b).is_le() {
                sum += i + 1;
            }
        }
        sum.to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let packets = input.iter().fold(vec![], |mut acc, (a, b)| {
            acc.push(a);
            acc.push(b);
            acc
        });
        let two = Value::List(vec![Value::List(vec![Value::Number(2)])]);
        let six = Value::List(vec![Value::List(vec![Value::Number(6)])]);

        let mut two_index = 1;
        let mut six_index = 2;

        for packet in packets.iter() {
            if packet.cmp(&two).is_le() {
                two_index += 1;
                six_index += 1;
            } else if packet.cmp(&six).is_le() {
                six_index += 1;
            }
        }

        (two_index * six_index).to_string()
    }

    fn parse_input(&self, input: &String) -> Self::Input {
        input
            .split("\n\n")
            .map(|x| {
                let mut split = x.split("\n");
                (
                    Value::parse(split.next().unwrap()),
                    Value::parse(split.next().unwrap()),
                )
            })
            .collect()
    }
}
