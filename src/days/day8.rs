use crate::days;

fn is_visible(x: usize, y: usize, input: &Vec<Vec<u8>>) -> bool {
    let height = input[y][x];
    let mut right = true;
    let mut left = true;
    let mut top = true;
    let mut bottom = true;
    for y in y + 1..input.len() {
        if input[y][x] >= height {
            bottom = false;
            break;
        }
    }
    for y in 0..y {
        if input[y][x] >= height {
            top = false;
            break;
        }
    }
    for x in x + 1..input[y].len() {
        if input[y][x] >= height {
            right = false;
            break;
        }
    }
    for x in 0..x {
        if input[y][x] >= height {
            left = false;
            break;
        }
    }
    right || left || top || bottom
}

fn scenic_score(x: usize, y: usize, input: &Vec<Vec<u8>>) -> u32 {
    let height = input[y][x];
    let mut right = 0;
    let mut left = 0;
    let mut top = 0;
    let mut bottom = 0;
    for y in y + 1..input.len() {
        bottom += 1;
        if input[y][x] >= height {
            break;
        }
    }
    for y in (0..y).rev() {
        top += 1;
        if input[y][x] >= height {
            break;
        }
    }
    for x in x + 1..input[y].len() {
        right += 1;
        if input[y][x] >= height {
            break;
        }
    }
    for x in (0..x).rev() {
        left += 1;
        if input[y][x] >= height {
            break;
        }
    }
    right * left * top * bottom
}

pub struct Day {}

impl days::Day for Day {
    type Input = Vec<Vec<u8>>;

    fn get_num(&self) -> u8 {
        8
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        let mut sum = 0;
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if is_visible(x, y, &input) {
                    sum += 1;
                }
            }
        }
        (sum.to_string(), true)
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        let mut max_score = 0;
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                let score = scenic_score(x, y, &input);
                if score > max_score {
                    max_score = score;
                }
            }
        }
        (max_score.to_string(), true)
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect()
    }
}
