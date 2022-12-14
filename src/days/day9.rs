use std::collections::HashSet;

use crate::days;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn move_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }

    fn is_touching(&self, other: &Position) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    fn follow(&mut self, other: &Position) {
        if self.is_touching(&other) {
            return;
        }
        if self.x == other.x {
            if self.y < other.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        } else if self.y == other.y {
            if self.x < other.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        } else {
            if self.x < other.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
            if self.y < other.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        }
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = Vec<(Direction, u32)>;

    fn get_num(&self) -> u8 {
        9
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> String {
        let mut head = Position::new();
        let mut tail = Position::new();
        let mut tail_path = HashSet::new();

        for (dir, dist) in input {
            for _ in 0..*dist {
                head.move_dir(dir);
                tail.follow(&head);
                tail_path.insert((tail.x, tail.y));
            }
        }

        tail_path.len().to_string()
    }

    fn part2(&mut self, input: &Self::Input) -> String {
        let mut head = Position::new();
        let mut tail1 = Position::new();
        let mut tail2 = Position::new();
        let mut tail3 = Position::new();
        let mut tail4 = Position::new();
        let mut tail5 = Position::new();
        let mut tail6 = Position::new();
        let mut tail7 = Position::new();
        let mut tail8 = Position::new();
        let mut tail9 = Position::new();
        let mut tail_path = HashSet::new();

        for (dir, dist) in input {
            for _ in 0..*dist {
                head.move_dir(dir);
                tail1.follow(&head);
                tail2.follow(&tail1);
                tail3.follow(&tail2);
                tail4.follow(&tail3);
                tail5.follow(&tail4);
                tail6.follow(&tail5);
                tail7.follow(&tail6);
                tail8.follow(&tail7);
                tail9.follow(&tail8);
                tail_path.insert((tail9.x, tail9.y));
            }
        }

        tail_path.len().to_string()
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input.lines().map(|x| {
            let mut split = x.split(" ");
            let dir = Direction::from_char(split.next().unwrap().chars().next().unwrap()).unwrap();
            let dist = split.next().unwrap().parse::<u32>().unwrap();
            (dir, dist)
        }).collect()
    }
}
