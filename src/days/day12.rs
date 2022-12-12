use core::panic;
use std::collections::HashMap;

use pathfinding::directed::astar::astar;

use crate::days;

fn char2elevation(c: char) -> usize {
    match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        'j' => 9,
        'k' => 10,
        'l' => 11,
        'm' => 12,
        'n' => 13,
        'o' => 14,
        'p' => 15,
        'q' => 16,
        'r' => 17,
        's' => 18,
        't' => 19,
        'u' => 20,
        'v' => 21,
        'w' => 22,
        'x' => 23,
        'y' => 24,
        'z' => 25,
        _ => panic!("Invalid char"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn neighbors(&self, grid: &Vec<Vec<usize>>) -> Vec<(Pos, usize)> {
        let x = self.x;
        let y = self.y;
        let next_elevation = grid[x][y] + 1;
        let mut neighbors = vec![];
        if x > 0 && grid[x - 1][y] <= next_elevation {
            neighbors.push(Pos::new(x - 1, y));
        }
        if x < grid.len() - 1 && grid[x + 1][y] <= next_elevation {
            neighbors.push(Pos::new(x + 1, y));
        }
        if y > 0 && grid[x][y - 1] <= next_elevation {
            neighbors.push(Pos::new(x, y - 1));
        }
        if y < grid[0].len() - 1 && grid[x][y + 1] <= next_elevation {
            neighbors.push(Pos::new(x, y + 1));
        }
        neighbors.into_iter().map(|p| (p, 1)).collect()
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = (Pos, Pos, Vec<Vec<usize>>);

    fn get_num(&self) -> u8 {
        12
    }

    fn part1(&self, input: &Self::Input) -> String {
        let (start, end, grid) = input;

        astar(
            start,
            |&current| current.neighbors(&grid),
            |&current| current.distance(*end),
            |&current| current == *end,
        )
        .unwrap()
        .1
        .to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let (_, end, grid) = input;
        let mut distances = HashMap::new();

        for (i, line) in grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == 0 {
                    let start = Pos::new(i, j);
                    if distances.contains_key(&start) {
                        continue;
                    }
                    match astar(
                        &start,
                        |&current| current.neighbors(&grid),
                        |&current| current.distance(*end),
                        |&current| current == *end,
                    ) {
                        Some((path, length)) => {
                            for (i, p) in path.into_iter().enumerate() {
                                if grid[p.x][p.y] == 0 {
                                    distances.insert(p, length - i);
                                }
                            }
                        },
                        None => (),
                    }
                }
            }
        }

        distances.values().min().unwrap().to_string()
    }

    fn parse_input(&self, input: &String) -> Self::Input {
        let mut start = Pos::new(0, 0);
        let mut end = Pos::new(0, 0);
        let mut grid: Vec<Vec<usize>> = vec![];
        for (i, line) in input.lines().enumerate() {
            grid.push(vec![]);
            for (j, c) in line.chars().enumerate() {
                grid[i].push(match c {
                    'S' => {
                        start = Pos::new(i, j);
                        0
                    }
                    'E' => {
                        end = Pos::new(i, j);
                        25
                    }
                    _ => char2elevation(c),
                })
            }
        }
        (start, end, grid)
    }
}
