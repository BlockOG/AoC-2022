use std::{collections::HashSet, ops::Add, str::FromStr};

use crate::days;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        let z = split.next().unwrap().parse().unwrap();
        Ok(Self::new(x, y, z))
    }
}

impl Add<&Pos> for Pos {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

const NEIGHBOR_DELTAS: [Pos; 6] = [
    Pos { x: -1, y: 0, z: 0 },
    Pos { x: 1, y: 0, z: 0 },
    Pos { x: 0, y: -1, z: 0 },
    Pos { x: 0, y: 1, z: 0 },
    Pos { x: 0, y: 0, z: -1 },
    Pos { x: 0, y: 0, z: 1 },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    grid: HashSet<Pos>,
}

impl Grid {
    fn surface_area(&self) -> usize {
        let mut surface_area = 0;
        for pos in &self.grid {
            for delta in &NEIGHBOR_DELTAS {
                if !self.grid.contains(&(*pos + delta)) {
                    surface_area += 1;
                }
            }
        }
        surface_area
    }

    fn outer_surface_area(&self) -> usize {
        let mut grid = HashSet::new();

        for x in self.min_x() - 1..=self.max_x() + 1 {
            for y in self.min_y() - 1..=self.max_y() + 1 {
                for z in self.min_z() - 1..=self.max_z() + 1 {
                    grid.insert(Pos::new(x, y, z));
                }
            }
        }

        let mut to_see = HashSet::new();
        to_see.insert(Pos::new(
            self.min_x() - 1,
            self.min_y() - 1,
            self.min_z() - 1,
        ));
        let mut been = HashSet::new();
        while !to_see.is_empty() {
            let mut new_to_see = HashSet::new();
            for pos in to_see.drain() {
                been.insert(pos);
                grid.remove(&pos);
                for delta in &NEIGHBOR_DELTAS {
                    let new_pos = pos + delta;
                    if !been.contains(&new_pos) && self.is_within_bounds(&new_pos) && !self.grid.contains(&new_pos) {
                        new_to_see.insert(new_pos);
                    }
                }
            }
            to_see = new_to_see;
        }

        Self { grid }.surface_area()
    }

    fn is_within_bounds(&self, pos: &Pos) -> bool {
        pos.x >= self.min_x() - 1
            && pos.x <= self.max_x() + 1
            && pos.y >= self.min_y() - 1
            && pos.y <= self.max_y() + 1
            && pos.z >= self.min_z() - 1
            && pos.z <= self.max_z() + 1
    }

    fn min_x(&self) -> i32 {
        self.grid.iter().map(|p| p.x).min().unwrap()
    }

    fn max_x(&self) -> i32 {
        self.grid.iter().map(|p| p.x).max().unwrap()
    }

    fn min_y(&self) -> i32 {
        self.grid.iter().map(|p| p.y).min().unwrap()
    }

    fn max_y(&self) -> i32 {
        self.grid.iter().map(|p| p.y).max().unwrap()
    }

    fn min_z(&self) -> i32 {
        self.grid.iter().map(|p| p.z).min().unwrap()
    }

    fn max_z(&self) -> i32 {
        self.grid.iter().map(|p| p.z).max().unwrap()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashSet::new();
        for line in s.lines() {
            grid.insert(line.parse().unwrap());
        }
        Ok(Self { grid })
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = Grid;

    fn get_num(&self) -> u8 {
        18
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> String {
        input.surface_area().to_string()
    }

    fn part2(&mut self, input: &Self::Input) -> String {
        input.outer_surface_area().to_string()
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input.parse().unwrap()
    }
}
