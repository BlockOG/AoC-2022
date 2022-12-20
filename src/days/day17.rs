use std::collections::HashMap;
use std::ops::{Add, AddAssign, Sub};

use crate::days;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn collides(&self, occupied: &HashMap<Pos, (usize, usize)>) -> bool {
        if self.y < 0 {
            return true;
        }
        if self.x < 0 || self.x > 6 {
            return true;
        }
        occupied.contains_key(self)
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<&Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: &Pos) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<&Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: &Pos) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add<Direction> for Pos {
    type Output = Pos;

    fn add(self, rhs: Direction) -> Self::Output {
        let mut pos = self;
        pos.dir(&rhs);
        pos
    }
}

impl Add<&Direction> for Pos {
    type Output = Pos;

    fn add(self, rhs: &Direction) -> Self::Output {
        let mut pos = self;
        pos.dir(rhs);
        pos
    }
}

impl AddAssign<Direction> for Pos {
    fn add_assign(&mut self, rhs: Direction) {
        self.dir(&rhs);
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::Left => '<',
            Direction::Right => '>',
            _ => panic!("Invalid direction"),
        }
    }
}

fn max_y(shape: &Vec<Pos>) -> i32 {
    shape.iter().map(|p| p.y).max().unwrap()
}

fn repeats(
    pos: &Pos,
    shape: &Vec<Pos>,
    occupied: &HashMap<Pos, (usize, usize)>,
) -> Option<(usize, usize)> {
    'offset: for offset in 0..pos.y {
        let jet_index1 = occupied.get(&Pos::new(shape[0].x + pos.x, shape[0].y + pos.y - offset));
        let jet_index2 = occupied.get(&Pos::new(
            shape[0].x + pos.x,
            shape[0].y + pos.y - (offset * 2),
        ));
        for p in shape.iter() {
            let p1 = Pos::new(p.x + pos.x, p.y + pos.y - offset);
            let p2 = Pos::new(p.x + pos.x, p.y + pos.y - (offset * 2));
            if !occupied.contains_key(&p1)
                || !occupied.contains_key(&p2)
                || occupied.get(&p1).unwrap().1 != jet_index1.unwrap().1
                || occupied.get(&p2).unwrap().1 != jet_index2.unwrap().1
                || occupied.get(&p2).unwrap().1 != jet_index1.unwrap().1
            {
                continue 'offset;
            }
        }
        return Some((
            offset as usize,
            occupied[&Pos::new(shape[0].x + pos.x, shape[0].y + pos.y - offset)].0,
        ));
    }
    None
}

fn height(rock_count: usize, jet_pattern: &Vec<Direction>) -> usize {
    let shapes = [
        vec![
            //  0123
            // 0████
            Pos::new(0, 0),
            Pos::new(1, 0),
            Pos::new(2, 0),
            Pos::new(3, 0),
        ],
        vec![
            //  012
            // 0 █
            // 1███
            // 2 █
            Pos::new(1, 0),
            Pos::new(0, 1),
            Pos::new(1, 1),
            Pos::new(2, 1),
            Pos::new(1, 2),
        ],
        vec![
            //  012
            // 0  █
            // 1  █
            // 2███
            Pos::new(0, 0),
            Pos::new(1, 0),
            Pos::new(2, 0),
            Pos::new(2, 1),
            Pos::new(2, 2),
        ],
        vec![
            //  0
            // 0█
            // 1█
            // 2█
            // 3█
            Pos::new(0, 0),
            Pos::new(0, 1),
            Pos::new(0, 2),
            Pos::new(0, 3),
        ],
        vec![
            //  01
            // 0██
            // 1██
            Pos::new(0, 0),
            Pos::new(1, 0),
            Pos::new(0, 1),
            Pos::new(1, 1),
        ],
    ];

    let mut occupied = HashMap::new();
    let mut pos = Pos::new(0, 0);
    let mut height = 0;
    let mut jet_pattern_index = (0..jet_pattern.len()).cycle().peekable();
    let mut rock_i = 0;
    let mut height_addition = 0;

    for shape in shapes.iter().cycle() {
        pos.x = 2;
        pos.y = height + 3;
        loop {
            match jet_pattern[jet_pattern_index.next().unwrap()] {
                Direction::Right => {
                    pos += Direction::Right;
                    if shape
                        .iter()
                        .map(|&p| p + pos)
                        .any(|p| p.collides(&occupied))
                    {
                        pos += Direction::Left;
                    }
                }
                Direction::Left => {
                    pos += Direction::Left;
                    if shape
                        .iter()
                        .map(|&p| p + pos)
                        .any(|p| p.collides(&occupied))
                    {
                        pos += Direction::Right;
                    }
                }
                _ => unreachable!("Jet pattern is only left and right"),
            }
            pos += Direction::Down;
            if shape
                .iter()
                .map(|&p| p + pos)
                .any(|p| p.collides(&occupied))
            {
                pos += Direction::Up;
                if height_addition > 0 {
                    break;
                }
                if let Some((offset, rock_i2)) = repeats(&pos, shape, &occupied) {
                    let repeat_amount_until_end = (rock_count - rock_i) / (rock_i - rock_i2);
                    rock_i += (rock_i - rock_i2) * repeat_amount_until_end;
                    height_addition = offset * repeat_amount_until_end;
                }
                break;
            }
        }
        shape.iter().for_each(|&p| {
            occupied.insert(p + pos, (rock_i, jet_pattern_index.peek().unwrap().clone()));
        });
        height = height.max(pos.y + max_y(shape) + 1);
        rock_i += 1;
        if rock_i >= rock_count {
            break;
        }
    }

    height as usize + height_addition
}

pub struct Day {}

impl days::Day for Day {
    type Input = Vec<Direction>;

    fn get_num(&self) -> u8 {
        17
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        (height(2022, input).to_string(), true)
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        (height(1_000_000_000_000, input).to_string(), true)
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input.chars().map(|c| c.into()).collect()
    }
}
