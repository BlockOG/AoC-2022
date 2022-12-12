use core::panic;

use pathfinding::directed::astar::astar;

use crate::days;

fn next_chars(c: char) -> Vec<char> {
    match c {
        'a' => vec!['a', 'b'],
        'b' => vec!['a', 'b', 'c'],
        'c' => vec!['a', 'b', 'c', 'd'],
        'd' => vec!['a', 'b', 'c', 'd', 'e'],
        'e' => vec!['a', 'b', 'c', 'd', 'e', 'f'],
        'f' => vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'g' => vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
        'h' => vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
        'i' => vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'],
        'j' => vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'],
        'k' => vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'],
        'l' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        ],
        'm' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
        ],
        'n' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
        ],
        'o' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        ],
        'p' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        ],
        'q' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r',
        ],
        'r' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's',
        ],
        's' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't',
        ],
        't' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u',
        ],
        'u' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v',
        ],
        'v' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w',
        ],
        'w' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x',
        ],
        'x' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
        ],
        'y' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ],
        'z' => vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ],
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

    fn neighbors(&self, grid: Vec<Vec<char>>) -> Vec<(Pos, usize)> {
        let x = self.x;
        let y = self.y;
        let next_chars = next_chars(grid[x][y]);
        let mut neighbors = vec![];
        if x > 0 && next_chars.contains(&grid[x - 1][y]) {
            neighbors.push(Pos::new(x - 1, y));
        }
        if x < grid.len() - 1 && next_chars.contains(&grid[x + 1][y]) {
            neighbors.push(Pos::new(x + 1, y));
        }
        if y > 0 && next_chars.contains(&grid[x][y - 1]) {
            neighbors.push(Pos::new(x, y - 1));
        }
        if y < grid[0].len() - 1 && next_chars.contains(&grid[x][y + 1]) {
            neighbors.push(Pos::new(x, y + 1));
        }
        neighbors.into_iter().map(|p| (p, 1)).collect()
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = (Pos, Pos, Vec<Vec<char>>);

    fn get_num(&self) -> u8 {
        12
    }

    fn part1(&self, input: &Self::Input) -> String {
        let (start, end, grid) = input;

        astar(
            start,
            |&current| current.neighbors(grid.clone()),
            |&current| current.distance(*end),
            |&current| current == *end,
        )
        .unwrap()
        .1
        .to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let (_, end, grid) = input;
        let mut distances = vec![];

        for (i, line) in grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == 'a' {
                    let start = Pos::new(i, j);
                    match astar(
                        &start,
                        |&current| current.neighbors(grid.clone()),
                        |&current| current.distance(*end),
                        |&current| current == *end,
                    ) {
                        Some(path) => distances.push(path.1),
                        None => (),
                    }
                }
            }
        }

        distances.iter().min().unwrap().to_string()
    }

    fn parse_input(&self, input: &String) -> Self::Input {
        let mut start = Pos::new(0, 0);
        let mut end = Pos::new(0, 0);
        let mut grid: Vec<Vec<char>> = vec![];
        for (i, line) in input.lines().enumerate() {
            grid.push(vec![]);
            for (j, c) in line.chars().enumerate() {
                grid[i].push(match c {
                    'S' => {
                        start = Pos::new(i, j);
                        'a'
                    }
                    'E' => {
                        end = Pos::new(i, j);
                        'z'
                    }
                    _ => c,
                })
            }
        }
        (start, end, grid)
    }
}
