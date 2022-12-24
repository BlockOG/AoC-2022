use std::{ops::Index, str::FromStr};

use pathfinding::prelude::astar;

use crate::days;

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::UP,
            'v' => Direction::DOWN,
            '<' => Direction::LEFT,
            '>' => Direction::RIGHT,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_in(&mut self, dir: &Direction) {
        match dir {
            Direction::UP => self.y -= 1,
            Direction::DOWN => self.y += 1,
            Direction::LEFT => self.x -= 1,
            Direction::RIGHT => self.x += 1,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Pos3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Blizzard {
    pos: Pos,
    dir: Direction,
}

impl Blizzard {
    fn new(pos: Pos, dir: Direction) -> Self {
        Self { pos, dir }
    }

    fn next(&mut self) {
        self.pos.move_in(&self.dir);
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Blizzards {
    blizzards: Vec<Blizzard>,
    width: i32,
    height: i32,
}

impl FromStr for Blizzards {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Skip the first and last line
        let width = lines.next().unwrap().len() as i32 - 2;
        lines.next_back();

        let mut blizzards = Vec::new();
        let mut height = 0;

        for (y, line) in lines.enumerate() {
            let mut chars = line.chars();

            // Skip the first and last character
            chars.next();
            chars.next_back();

            for (x, c) in chars.enumerate() {
                if c == '.' {
                    continue;
                }

                blizzards.push(Blizzard::new(Pos::new(x as i32, y as i32), c.into()));
            }
            height += 1;
        }

        Ok(Self {
            blizzards,
            width,
            height,
        })
    }
}

impl Blizzards {
    fn next(&mut self) {
        for blizzard in self.blizzards.iter_mut() {
            blizzard.next();

            // Wrap around
            blizzard.pos.x = blizzard.pos.x.rem_euclid(self.width);
            blizzard.pos.y = blizzard.pos.y.rem_euclid(self.height);
        }
    }
}

struct Vec3D<T> {
    data: Vec<Vec<Vec<T>>>,
    width: i32,
    height: i32,
    depth: i32,
}

impl<T> Vec3D<T> {
    fn new(width: i32, height: i32) -> Self {
        Self {
            data: Vec::new(),
            width,
            height,
            depth: 0,
        }
    }

    fn add(&mut self, vec2d: Vec<Vec<T>>) {
        self.depth += 1;
        self.data.push(vec2d);
    }
}

impl<T> Index<&Pos3D> for Vec3D<T> {
    type Output = T;

    fn index(&self, pos: &Pos3D) -> &Self::Output {
        &self.data[pos.z.rem_euclid(self.depth) as usize][pos.y as usize][pos.x as usize]
    }
}

pub struct Grid {
    accessible: Vec3D<bool>,
}

impl Grid {
    fn new(blizzards: &Blizzards) -> Self {
        let mut grid = Self {
            accessible: Vec3D::new(blizzards.width, blizzards.height),
        };
        grid.add(blizzards);
        grid
    }

    fn add(&mut self, blizzards: &Blizzards) {
        let mut vec2d = vec![vec![true; blizzards.width as usize]; blizzards.height as usize];
        for blizzard in blizzards.blizzards.iter() {
            vec2d[blizzard.pos.y as usize][blizzard.pos.x as usize] = false;
        }
        self.accessible.add(vec2d);
    }

    fn neighbors(&self, pos: &Pos3D) -> Vec<(Pos3D, i32)> {
        let mut neighbors = Vec::new();

        // Start and finish
        if pos.y <= 0 && pos.x == 0 {
            neighbors.push((Pos3D::new(0, -1, pos.z + 1), 1));
        }
        if pos.y >= self.accessible.height - 1 && pos.x == self.accessible.width - 1 {
            neighbors.push((
                Pos3D::new(self.accessible.width - 1, self.accessible.height, pos.z + 1),
                1,
            ));
        }

        if pos.x >= 0 && pos.x < self.accessible.width {
            // Move up
            if pos.y > 0 && self.accessible[&Pos3D::new(pos.x, pos.y - 1, pos.z + 1)] {
                neighbors.push((Pos3D::new(pos.x, pos.y - 1, pos.z + 1), 1));
            }

            // Move down
            if pos.y < self.accessible.height - 1
                && self.accessible[&Pos3D::new(pos.x, pos.y + 1, pos.z + 1)]
            {
                neighbors.push((Pos3D::new(pos.x, pos.y + 1, pos.z + 1), 1));
            }
        }

        if pos.y >= 0 && pos.y < self.accessible.height {
            // Move left
            if pos.x > 0 && self.accessible[&Pos3D::new(pos.x - 1, pos.y, pos.z + 1)] {
                neighbors.push((Pos3D::new(pos.x - 1, pos.y, pos.z + 1), 1));
            }

            // Move right
            if pos.x < self.accessible.width - 1
                && self.accessible[&Pos3D::new(pos.x + 1, pos.y, pos.z + 1)]
            {
                neighbors.push((Pos3D::new(pos.x + 1, pos.y, pos.z + 1), 1));
            }
        }

        // Stay in place
        if pos.x >= 0
            && pos.x < self.accessible.width
            && pos.y >= 0
            && pos.y < self.accessible.height
        {
            if self.accessible[&Pos3D::new(pos.x, pos.y, pos.z + 1)] {
                neighbors.push((Pos3D::new(pos.x, pos.y, pos.z + 1), 1));
            }
        }

        neighbors
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
        (
            astar(
                &Pos3D::new(0, -1, 0),
                |p| input.neighbors(p),
                |p| {
                    (p.x.abs_diff(input.accessible.width - 1)
                        + p.y.abs_diff(input.accessible.height)) as i32
                },
                |p| p.x == input.accessible.width - 1 && p.y == input.accessible.height,
            )
            .unwrap()
            .1
            .to_string(),
            true,
        )
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        let mut z = 0;
        let first = astar(
            &Pos3D::new(0, -1, z),
            |p| input.neighbors(p),
            |p| {
                (p.x.abs_diff(input.accessible.width - 1) + p.y.abs_diff(input.accessible.height))
                    as i32
            },
            |p| {
                let success = p.x == input.accessible.width - 1 && p.y == input.accessible.height;
                if success {
                    z = p.z;
                }
                success
            },
        )
        .unwrap()
        .1;
        let second = astar(
            &Pos3D::new(input.accessible.width - 1, input.accessible.height, z),
            |p| input.neighbors(p),
            |p| (p.x.abs_diff(0) + p.y.abs_diff(-1)) as i32,
            |p| {
                let success = p.x == 0 && p.y == -1;
                if success {
                    z = p.z;
                }
                success
            },
        )
        .unwrap()
        .1;
        let third = astar(
            &Pos3D::new(0, -1, z),
            |p| input.neighbors(p),
            |p| {
                (p.x.abs_diff(input.accessible.width - 1) + p.y.abs_diff(input.accessible.height))
                    as i32
            },
            |p| {
                let success = p.x == input.accessible.width - 1 && p.y == input.accessible.height;
                if success {
                    z = p.z;
                }
                success
            },
        )
        .unwrap()
        .1;
        ((first + second + third).to_string(), true)
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        let mut blizzards: Blizzards = input.parse().unwrap();
        let original_blizzards = blizzards.clone();
        let mut grid = Grid::new(&blizzards);
        blizzards.next();

        while blizzards != original_blizzards {
            grid.add(&blizzards);
            blizzards.next();
        }

        grid
    }
}
