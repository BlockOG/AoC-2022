use pathfinding::prelude::{astar, dijkstra};

use crate::days;

fn char2elevation(c: char) -> usize {
    c as usize - 'a' as usize
}

#[inline]
fn index_grid(index: &Pos, grid: &(Vec<usize>, usize, usize)) -> usize {
    grid.0[index.x + index.y * grid.1]
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

    fn predecessors(&self, grid: &(Vec<usize>, usize, usize)) -> Vec<(Pos, usize)> {
        let next_elevation = index_grid(self, grid) - 1;
        let mut neighbors = vec![];
        if self.x > 0 && index_grid(&self.sub_x(1), grid) >= next_elevation {
            neighbors.push((Pos::new(self.x - 1, self.y), 1));
        }
        if self.x < grid.1 - 1 && index_grid(&self.add_x(1), grid) >= next_elevation {
            neighbors.push((Pos::new(self.x + 1, self.y), 1));
        }
        if self.y > 0 && index_grid(&self.sub_y(1), grid) >= next_elevation {
            neighbors.push((Pos::new(self.x, self.y - 1), 1));
        }
        if self.y < grid.2 - 1 && index_grid(&self.add_y(1), grid) >= next_elevation {
            neighbors.push((Pos::new(self.x, self.y + 1), 1));
        }
        neighbors
    }

    fn add_x(&self, x: usize) -> Self {
        Self {
            x: self.x + x,
            y: self.y,
        }
    }

    fn sub_x(&self, x: usize) -> Self {
        Self {
            x: self.x - x,
            y: self.y,
        }
    }

    fn add_y(&self, y: usize) -> Self {
        Self {
            x: self.x,
            y: self.y + y,
        }
    }

    fn sub_y(&self, y: usize) -> Self {
        Self {
            x: self.x,
            y: self.y - y,
        }
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = (Pos, Pos, (Vec<usize>, usize, usize));

    fn get_num(&self) -> u8 {
        12
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> String {
        let (start, end, grid) = input;

        astar(
            end,
            |&current| current.predecessors(&grid),
            |&current| current.distance(*start),
            |&current| current == *start,
        )
        .unwrap()
        .1
        .to_string()
    }

    fn part2(&mut self, input: &Self::Input) -> String {
        let (_, end, grid) = input;

        dijkstra(
            end,
            |&current| current.predecessors(&grid),
            |current| index_grid(current, grid) == 0,
        )
        .unwrap()
        .1
        .to_string()
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        let mut start = Pos::new(0, 0);
        let mut end = Pos::new(0, 0);
        let mut grid: Vec<usize> = vec![];
        let mut width = 0;
        let height = input.lines().count();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.push(match c {
                    'S' => {
                        start = Pos::new(x, y);
                        0
                    }
                    'E' => {
                        end = Pos::new(x, y);
                        25
                    }
                    _ => char2elevation(c),
                });
                width = x + 1;
            }
        }
        (start, end, (grid, width, height))
    }
}
