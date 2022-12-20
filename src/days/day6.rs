use crate::days;

pub struct Day {}

impl days::Day for Day {
    type Input = String;

    fn get_num(&self) -> u8 {
        6
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        for i in 0..input.len() - 4 {
            let mut group = String::new();
            for j in input.get(i..i + 4).unwrap().chars() {
                if group.contains(j) {
                    break;
                }
                group.push(j);
                if group.len() == 4 {
                    return (format!("{}", i + 4), true);
                }
            }
        }
        ("0".to_string(), false)
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        for i in 0..input.len() - 14 {
            let mut group = String::new();
            for j in input.get(i..i + 14).unwrap().chars() {
                if group.contains(j) {
                    break;
                }
                group.push(j);
                if group.len() == 14 {
                    return (format!("{}", i + 14), true);
                }
            }
        }
        ("0".to_string(), false)
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input.clone()
    }
}
