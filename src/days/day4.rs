use crate::days;

fn contains_other(a: &(i32, i32), b: &(i32, i32)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn any_contains_other(a: &(i32, i32), b: &(i32, i32)) -> bool {
    contains_other(a, b) || contains_other(b, a)
}

fn overlaps(a: &(i32, i32), b: &(i32, i32)) -> bool {
    a.0 <= b.0 && a.1 >= b.0 || b.0 <= a.0 && b.1 >= a.0
}

pub struct Day {}

impl days::Day for Day {
    type Input = Vec<((i32, i32), (i32, i32))>;

    fn get_num(&self) -> u8 {
        4
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> String {
        let mut contained = 0;
        for (a, b) in input.iter() {
            if any_contains_other(a, b) {
                contained += 1;
            }
        }
        contained.to_string()
    }

    fn part2(&mut self, input: &Self::Input) -> String {
        let mut overlapped = 0;
        for (a, b) in input.iter() {
            if overlaps(a, b) {
                overlapped += 1;
            }
        }
        overlapped.to_string()
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input
            .lines()
            .map(|s| {
                let mut range_split = s.split(",");
                let mut a_split = range_split.next().unwrap().split("-");
                let mut b_split = range_split.next().unwrap().split("-");
                (
                    (
                        a_split.next().unwrap().parse().unwrap(),
                        a_split.next().unwrap().parse().unwrap(),
                    ),
                    (
                        b_split.next().unwrap().parse().unwrap(),
                        b_split.next().unwrap().parse().unwrap(),
                    ),
                )
            })
            .collect()
    }
}
