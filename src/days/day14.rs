use crate::days;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    fn down_left(&self) -> Self {
        Self::new(self.x - 1, self.y + 1)
    }

    fn down_right(&self) -> Self {
        Self::new(self.x + 1, self.y + 1)
    }

    fn index(&self) -> usize {
        (self.y * 1000 + self.x) as usize
    }

    fn new_index(x: i32, y: i32) -> usize {
        (y * 1000 + x) as usize
    }
}

fn has_block(blocks: &Vec<bool>, max_y: i32, block: &Pos) -> bool {
    if block.y >= max_y + 2 {
        return false;
    }
    blocks[block.index()]
}

pub struct Day {
    blocks: Vec<bool>,
    max_y: i32,
    rested_sand: i32,
}

impl days::Day for Day {
    type Input = ();

    fn get_num(&self) -> u8 {
        14
    }

    fn new() -> Self {
        Self {
            blocks: vec![true; 1000 * 1000],
            max_y: 0,
            rested_sand: 0,
        }
    }

    fn part1(&mut self, _input: &Self::Input) -> String {
        let mut sand = Pos::new(500, 0);
        while sand.y <= self.max_y {
            if self.blocks[sand.down().index()] {
                sand = sand.down();
            } else if self.blocks[sand.down_left().index()] {
                sand = sand.down_left();
            } else if self.blocks[sand.down_right().index()] {
                sand = sand.down_right();
            } else {
                self.rested_sand += 1;
                self.blocks[sand.index()] = false;
                sand = Pos::new(500, 0);
            }
        }
        self.rested_sand.to_string()
    }

    fn part2(&mut self, _input: &Self::Input) -> String {
        let mut sand = Pos::new(500, 0);
        while self.blocks[500] {
            if has_block(&self.blocks, self.max_y, &sand.down()) {
                sand = sand.down();
            } else if has_block(&self.blocks, self.max_y, &sand.down_left()) {
                sand = sand.down_left();
            } else if has_block(&self.blocks, self.max_y, &sand.down_right()) {
                sand = sand.down_right();
            } else {
                self.rested_sand += 1;
                self.blocks[sand.index()] = false;
                sand = Pos::new(500, 0);
            }
        }
        self.rested_sand.to_string()
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        let paths = input
            .lines()
            .map(|x| {
                let mut poses = vec![];
                for i in x.split(" -> ") {
                    let mut pos = i.split(",");
                    poses.push(Pos::new(
                        pos.next().unwrap().parse().unwrap(),
                        pos.next().unwrap().parse().unwrap(),
                    ));
                }
                poses
            })
            .collect::<Vec<Vec<Pos>>>();

        for path in paths.iter() {
            for window in path.windows(2) {
                let (start, end) = (window[0], window[1]);
                let (mut x, mut y) = (start.x, start.y);
                let (x2, y2) = (end.x, end.y);
                if y > self.max_y {
                    self.max_y = y;
                }
                if y2 > self.max_y {
                    self.max_y = y2;
                }
                while x != x2 || y != y2 {
                    self.blocks[Pos::new_index(x, y)] = false;
                    if x < x2 {
                        x += 1;
                    } else if x > x2 {
                        x -= 1;
                    }
                    if y < y2 {
                        y += 1;
                    } else if y > y2 {
                        y -= 1;
                    }
                }
                self.blocks[Pos::new_index(x, y)] = false;
            }
        }
    }
}
