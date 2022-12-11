use itertools::Itertools;

use crate::days;

#[derive(Debug, Clone)]
enum Operation {
    Add(u128),
    Multiply(u128),
    Square,
}

#[derive(Debug, Clone)]
struct Test {
    divisor: u128,
    true_monkey: u8,
    false_monkey: u8,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test: Test,
}

pub struct Day {}

impl days::Day for Day {
    type Input = Vec<Monkey>;

    fn get_num(&self) -> u8 {
        11
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut monkeys = input.clone();
        let mut inspects = vec![0; monkeys.len()];
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let monkey = monkeys.get(i).unwrap().clone();
                for item in monkey.items.iter() {
                    let new_item = match monkey.operation {
                        Operation::Add(n) => item + n,
                        Operation::Multiply(n) => item * n,
                        Operation::Square => item * item,
                    } / 3;
                    inspects[i] += 1;
                    monkeys[i].items.remove(0);
                    if new_item % monkey.test.divisor == 0 {
                        monkeys[monkey.test.true_monkey as usize]
                            .items
                            .push(new_item);
                    } else {
                        monkeys[monkey.test.false_monkey as usize]
                            .items
                            .push(new_item);
                    }
                }
            }
        }
        inspects
            .iter()
            .sorted()
            .rev()
            .take(2)
            .product::<u32>()
            .to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut monkeys = input.clone();
        let test_lcm = monkeys.iter().map(|m| m.test.divisor).product::<u128>();
        let mut inspects = vec![0; monkeys.len()];
        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                let monkey = monkeys.get(i).unwrap().clone();
                for item in monkey.items.iter() {
                    let new_item = match monkey.operation {
                        Operation::Add(n) => item + n,
                        Operation::Multiply(n) => item * n,
                        Operation::Square => item * item,
                    } % test_lcm;
                    inspects[i] += 1;
                    monkeys[i].items.remove(0);
                    if new_item % monkey.test.divisor == 0 {
                        monkeys[monkey.test.true_monkey as usize]
                            .items
                            .push(new_item);
                    } else {
                        monkeys[monkey.test.false_monkey as usize]
                            .items
                            .push(new_item);
                    }
                }
            }
        }
        inspects
            .iter()
            .sorted()
            .rev()
            .take(2)
            .product::<u128>()
            .to_string()
    }

    fn parse_input(&self, input: &String) -> Self::Input {
        // Monkey 0:
        //     Starting items: 57, 58
        //     Operation: new = old * 19
        //     Test: divisible by 7
        //         If true: throw to monkey 2
        //         If false: throw to monkey 3

        // Monkey 1:
        //     Starting items: 66, 52, 59, 79, 94, 73
        //     Operation: new = old + 1
        //     Test: divisible by 19
        //         If true: throw to monkey 4
        //         If false: throw to monkey 6

        // Monkey 2:
        //     Starting items: 80
        //     Operation: new = old + 6
        //     Test: divisible by 5
        //         If true: throw to monkey 7
        //         If false: throw to monkey 5

        // Monkey 3:
        //     Starting items: 82, 81, 68, 66, 71, 83, 75, 97
        //     Operation: new = old + 5
        //     Test: divisible by 11
        //         If true: throw to monkey 5
        //         If false: throw to monkey 2

        // Monkey 4:
        //     Starting items: 55, 52, 67, 70, 69, 94, 90
        //     Operation: new = old * old
        //     Test: divisible by 17
        //         If true: throw to monkey 0
        //         If false: throw to monkey 3

        // Monkey 5:
        //     Starting items: 69, 85, 89, 91
        //     Operation: new = old + 7
        //     Test: divisible by 13
        //         If true: throw to monkey 1
        //         If false: throw to monkey 7

        // Monkey 6:
        //     Starting items: 75, 53, 73, 52, 75
        //     Operation: new = old * 7
        //     Test: divisible by 2
        //         If true: throw to monkey 0
        //         If false: throw to monkey 4

        // Monkey 7:
        //     Starting items: 94, 60, 79
        //     Operation: new = old + 2
        //     Test: divisible by 3
        //         If true: throw to monkey 1
        //         If false: throw to monkey 6
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
