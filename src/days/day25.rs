use colored::Colorize;

use crate::days;

fn snafu_digit_to_i64(digit: char) -> i64 {
    match digit {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("{} is not a valid SNAFU digit!", digit),
    }
}

fn snafu_to_i64(num: String) -> i64 {
    let mut result = vec![];
    let mut multiple = 1;
    for digit in num.chars().rev() {
        result.push(snafu_digit_to_i64(digit) * multiple);
        multiple *= 5;
    }
    result.iter().sum()
}

fn i64_digit_to_snafu(digit: i64) -> String {
    match digit {
        -2 => "=".to_string(),
        -1 => "-".to_string(),
        0 => "0".to_string(),
        1 => "1".to_string(),
        2 => "2".to_string(),
        _ => panic!("{} is not a valid SNAFU digit!", digit),
    }
}

fn i64_to_snafu(num: i64) -> String {
	if num == 0 {
        "".to_string()
    } else {
        match num.rem_euclid(5) {
            0 | 1 | 2 => i64_to_snafu(num / 5) + &i64_digit_to_snafu(num.rem_euclid(5)),
            3 | 4 => i64_to_snafu(num / 5 + 1) + &i64_digit_to_snafu(num.rem_euclid(5) - 5),
            _ => panic!("How did you get here?!"),
        }
    }
}

pub struct Day {
    day_num: u8,
}

impl days::Day for Day {
    type Input = Vec<i64>;

    fn get_num(&self) -> u8 {
        self.day_num
    }

    fn new(day_num: u8) -> Self {
        Self { day_num }
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        (i64_to_snafu(input.iter().sum()), true)
    }

    fn part2(&mut self, _input: &Self::Input) -> (String, bool) {
        ("This is the last day! So no part 2".green().bold().to_string(), false)
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input
            .lines()
            .map(|line| snafu_to_i64(line.to_string()))
            .collect()
    }
}
