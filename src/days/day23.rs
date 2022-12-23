use std::{collections::HashSet, ops::Add, str::FromStr};

use crate::days;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Clone)]
pub struct Grid {
    grid: HashSet<Pos>,
    proposed_pos: Vec<(Pos, Pos)>,
    directions: Vec<[Pos; 3]>,
}

impl Grid {
    fn new() -> Self {
        Self {
            grid: HashSet::new(),
            proposed_pos: Vec::new(),
            directions: vec![
                [Pos::new(-1, -1), Pos::new(0, -1), Pos::new(1, -1)],
                [Pos::new(-1, 1), Pos::new(0, 1), Pos::new(1, 1)],
                [Pos::new(-1, 1), Pos::new(-1, 0), Pos::new(-1, -1)],
                [Pos::new(1, 1), Pos::new(1, 0), Pos::new(1, -1)],
            ],
        }
    }

    fn add(&mut self, pos: Pos) {
        self.grid.insert(pos);
    }

    fn contains(&self, pos: Pos) -> bool {
        self.grid.contains(&pos)
    }

    fn get_neighbours(&self, pos: &Pos) -> usize {
        let mut count = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                if self.contains(Pos::new(pos.x + x, pos.y + y)) {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_empty(&self) -> usize {
        let mut count = 0;
        for x in self.grid.iter().map(|p| p.x).min().unwrap()
            ..=self.grid.iter().map(|p| p.x).max().unwrap()
        {
            for y in self.grid.iter().map(|p| p.y).min().unwrap()
                ..=self.grid.iter().map(|p| p.y).max().unwrap()
            {
                if !self.contains(Pos::new(x, y)) {
                    count += 1;
                }
            }
        }
        count
    }

    fn proposed_positions(&self, pos: &Pos) -> Option<Pos> {
        if self.get_neighbours(pos) == 0 {
            return None;
        }
        for dir in self.directions.iter() {
            if !dir.iter().map(|d| pos.add(*d)).any(|p| self.contains(p)) {
                return Some(pos.add(dir[1]));
            }
        }
        None
    }

    fn propose_positions(&mut self) {
        self.proposed_pos.clear();
        for pos in self.grid.iter() {
            if let Some(new_pos) = self.proposed_positions(&pos) {
                self.proposed_pos.push((pos.clone(), new_pos));
            } else {
                self.proposed_pos.push((pos.clone(), pos.clone()));
            }
        }
    }

    fn apply_proposed_positions(&mut self) -> bool {
        let mut new_grid = HashSet::new();
        let mut changed = false;
        for i in 0..self.proposed_pos.len() {
            let (old_pos, new_pos) = self.proposed_pos.get(i).unwrap().clone();
            if self.proposed_pos.iter().any(|(o, p)| o != &old_pos && p == &new_pos) {
                new_grid.insert(old_pos.clone());
            } else {
                new_grid.insert(new_pos.clone());
                if old_pos != new_pos {
                    changed = true;
                }
            }
        }
        self.grid = new_grid;
        changed
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    grid.add(Pos::new(x as i32, y as i32));
                }
            }
        }
        Ok(grid)
    }
}

pub struct Day {
    day_num: u8,
}

impl days::Day for Day {
    type Input = Grid;

    fn get_num(&self) -> u8 {
        self.day_num
    }

    fn new(day_num: u8) -> Self {
        Self { day_num }
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        let mut grid = input.clone();

        for _ in 0..10 {
            grid.propose_positions();
            grid.apply_proposed_positions();
            grid.directions.rotate_left(1);
        }

        (grid.count_empty().to_string(), true)
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        let mut grid = input.clone();
        let mut round = 0;

        loop {
            round += 1;
            grid.propose_positions();
            if !grid.apply_proposed_positions() {
                break;
            }
            grid.directions.rotate_left(1);
        }

        (round.to_string(), true)
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input.parse().unwrap()
    }
}
