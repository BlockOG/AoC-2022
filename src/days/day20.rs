use std::str::FromStr;

use crate::days;

#[derive(Clone)]
pub struct Nums {
    nums: Vec<i64>,
    nums_indexes: Vec<usize>,
}

impl Nums {
    fn new() -> Self {
        Self {
            nums: vec![],
            nums_indexes: vec![],
        }
    }

    fn add(&mut self, num: i64) {
        self.nums_indexes.push(self.nums.len());
        self.nums.push(num);
    }

    fn get(&self, index: usize) -> i64 {
        self.nums[index % self.nums.len()]
    }

    fn get_by_0(&self, index: usize) -> i64 {
        self.get(self.nums.iter().position(|x| x == &0).unwrap() + index)
    }

    fn move_delta(&mut self, original_index: usize, delta: &i64) {
        let nums_index = self
            .nums_indexes
            .iter()
            .position(|x| x == &original_index)
            .unwrap();
        let new_nums_index =
            ((nums_index as i64 + delta).rem_euclid(self.nums.len() as i64 - 1)) as usize;

        let num = self.nums.remove(nums_index);
        let nums_index = self.nums_indexes.remove(nums_index);
        self.nums.insert(new_nums_index, num);
        self.nums_indexes.insert(new_nums_index, nums_index);
    }
}

impl FromStr for Nums {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = Self::new();
        for line in s.lines() {
            nums.add(line.parse().unwrap());
        }
        Ok(nums)
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = Nums;

    fn get_num(&self) -> u8 {
        20
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        let mut nums = input.clone();
        for (i, j) in input.nums.iter().enumerate() {
            nums.move_delta(i, j);
        }
        (
            (nums.get_by_0(1000) + nums.get_by_0(2000) + nums.get_by_0(3000)).to_string(),
            true,
        )
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        let mut nums = input.clone();
        for i in nums.nums.iter_mut() {
            *i *= 811589153;
        }
        for _ in 0..10 {
            for (i, j) in input.nums.iter().enumerate() {
                nums.move_delta(i, &(j * 811589153));
            }
        }
        (
            (nums.get_by_0(1000) + nums.get_by_0(2000) + nums.get_by_0(3000)).to_string(),
            true,
        )
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input.parse().unwrap()
    }
}
