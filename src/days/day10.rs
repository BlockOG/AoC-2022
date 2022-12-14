use crate::days;

pub enum Instruction {
    Noop,      // example: noop
    AddX(i32), // example: addx 10
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let instruction = parts.next().unwrap();
        match instruction {
            "noop" => Self::Noop,
            "addx" => Self::AddX(parts.next().unwrap().parse().unwrap()),
            _ => panic!("Unknown instruction"),
        }
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = Vec<Instruction>;

    fn get_num(&self) -> u8 {
        10
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> String {
        let mut signal_strenghs = 0;
        let mut cycle = 1;
        let mut x = 1;
        for instruction in input.iter() {
            cycle += 1;
            if (cycle + 20) % 40 == 0 {
                signal_strenghs += cycle * x;
            }
            match instruction {
                Instruction::Noop => (),
                Instruction::AddX(n) => {
                    x += n;
                    cycle += 1;
                    if (cycle + 20) % 40 == 0 {
                        signal_strenghs += cycle * x;
                    }
                }
            }
        }
        signal_strenghs.to_string()
    }

    fn part2(&mut self, input: &Self::Input) -> String {
        let mut cycle = 1;
        let mut x: i32 = 1;
        let mut crt = vec![String::new()];
        for instruction in input.iter() {
            cycle += 1;
            if (crt.last().unwrap().len() as i32).abs_diff(x) <= 1 {
                crt.last_mut().unwrap().push('#');
            } else {
                crt.last_mut().unwrap().push(' ');
            }
            if (cycle - 1) % 40 == 0 {
                crt.push(String::new());
            }
            match instruction {
                Instruction::Noop => (),
                Instruction::AddX(n) => {
                    cycle += 1;
                    if (crt.last().unwrap().len() as i32).abs_diff(x) <= 1 {
                        crt.last_mut().unwrap().push('#');
                    } else {
                        crt.last_mut().unwrap().push(' ');
                    }
                    if (cycle - 1) % 40 == 0 {
                        crt.push(String::new());
                    }
                    x += n;
                }
            }
        }
        crt.pop();
        crt.iter().fold(String::new(), |acc, x| {
            acc + "\n" + &x.replace("#", "██").replace(" ", "  ")
        })
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input.lines().map(|line| Instruction::parse(line)).collect()
    }
}
