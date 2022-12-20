use itertools::Itertools;

use crate::days;

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Test {
    divisor: u64,
    true_monkey: u8,
    false_monkey: u8,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

pub struct Day {}

impl days::Day for Day {
    type Input = Vec<Monkey>;

    fn get_num(&self) -> u8 {
        11
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        let mut monkeys = input.clone();
        let mut inspects = [0; 8];
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                for item_index in (0..monkeys[i].items.len()).rev() {
                    let item = monkeys[i].items[item_index];
                    let new_item = match monkeys[i].operation {
                        Operation::Add(n) => item + n,
                        Operation::Multiply(n) => item * n,
                        Operation::Square => item * item,
                    } / 3;
                    inspects[i] += 1;
                    monkeys[i].items.pop();
                    if new_item % monkeys[i].test.divisor == 0 {
                        let true_monkey = monkeys[i].test.true_monkey as usize;
                        monkeys[true_monkey].items.push(new_item);
                    } else {
                        let false_monkey = monkeys[i].test.false_monkey as usize;
                        monkeys[false_monkey].items.push(new_item);
                    }
                }
            }
        }
        (
            inspects
                .iter()
                .sorted()
                .rev()
                .take(2)
                .product::<u64>()
                .to_string(),
            true,
        )
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        let mut monkeys = input.clone();
        let test_lcm = monkeys.iter().map(|m| m.test.divisor).product::<u64>();
        let mut inspects = [0; 8];
        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                for item_index in (0..monkeys[i].items.len()).rev() {
                    let item = monkeys[i].items[item_index];
                    let new_item = match monkeys[i].operation {
                        Operation::Add(n) => item + n,
                        Operation::Multiply(n) => item * n,
                        Operation::Square => item * item,
                    } % test_lcm;
                    inspects[i] += 1;
                    monkeys[i].items.pop();
                    if new_item % monkeys[i].test.divisor == 0 {
                        let true_monkey = monkeys[i].test.true_monkey as usize;
                        monkeys[true_monkey].items.push(new_item);
                    } else {
                        let false_monkey = monkeys[i].test.false_monkey as usize;
                        monkeys[false_monkey].items.push(new_item);
                    }
                }
            }
        }
        (
            inspects
                .iter()
                .sorted()
                .rev()
                .take(2)
                .product::<u64>()
                .to_string(),
            true,
        )
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input
            .split("\n\n")
            .map(|monkey| {
                let mut lines = monkey.lines();
                lines.next();
                let starting_items = lines
                    .next()
                    .unwrap()
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .map(|item| item.parse().unwrap())
                    .collect();
                let operation = lines.next().unwrap().split(": ").nth(1).unwrap();
                let operation = if operation == "new = old * old" {
                    Operation::Square
                } else if operation.starts_with("new = old * ") {
                    Operation::Multiply(
                        operation
                            .split("new = old * ")
                            .nth(1)
                            .unwrap()
                            .parse()
                            .unwrap(),
                    )
                } else if operation.starts_with("new = old + ") {
                    Operation::Add(
                        operation
                            .split("new = old + ")
                            .nth(1)
                            .unwrap()
                            .parse()
                            .unwrap(),
                    )
                } else {
                    panic!("Unknown operation: {}", operation);
                };
                let test = lines.next().unwrap().split(": ").nth(1).unwrap();
                let test = if test.starts_with("divisible by ") {
                    let divisor = test.split("divisible by ").nth(1).unwrap().parse().unwrap();
                    let true_monkey = lines
                        .next()
                        .unwrap()
                        .split(": ")
                        .nth(1)
                        .unwrap()
                        .split("throw to monkey ")
                        .nth(1)
                        .unwrap()
                        .parse()
                        .unwrap();
                    let false_monkey = lines
                        .next()
                        .unwrap()
                        .split(": ")
                        .nth(1)
                        .unwrap()
                        .split("throw to monkey ")
                        .nth(1)
                        .unwrap()
                        .parse()
                        .unwrap();
                    Test {
                        divisor,
                        true_monkey,
                        false_monkey,
                    }
                } else {
                    panic!("Unknown test: {}", test);
                };
                Monkey {
                    items: starting_items,
                    operation,
                    test,
                }
            })
            .collect()
    }
}
