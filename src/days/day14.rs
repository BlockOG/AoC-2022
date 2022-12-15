use crate::days;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn follow(&self, went: Went) -> Self {
        match went {
            Went::Down => self.down(),
            Went::DownLeft => self.down_left(),
            Went::DownRight => self.down_right(),
        }
    }

    fn follow_rev(&self, went: Went) -> Self {
        match went {
            Went::Down => self.up(),
            Went::DownLeft => self.up_right(),
            Went::DownRight => self.up_left(),
        }
    }

    fn up(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    fn up_left(&self) -> Self {
        Self::new(self.x - 1, self.y - 1)
    }

    fn up_right(&self) -> Self {
        Self::new(self.x + 1, self.y - 1)
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
        Self::new_index(self.x, self.y)
    }

    #[inline]
    fn new_index(x: usize, y: usize) -> usize {
        x + y * 600
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Went {
    Down,
    DownLeft,
    DownRight,
}

impl Went {
    fn next(&self) -> Option<Self> {
        match self {
            Went::Down => Some(Went::DownLeft),
            Went::DownLeft => Some(Went::DownRight),
            Went::DownRight => None,
        }
    }
}

fn misses_block(blocks: &Vec<bool>, max_y: usize, block: &Pos) -> bool {
    if block.y >= max_y + 2 {
        return false;
    }
    blocks[block.index()]
}

pub struct Day {
    blocksp1: Vec<bool>,
    blocksp2: Vec<Vec<bool>>,

    max_y: usize,
}

impl days::Day for Day {
    type Input = ();

    fn get_num(&self) -> u8 {
        14
    }

    fn new() -> Self {
        Self {
            blocksp1: vec![true; 600 * 200],
            blocksp2: vec![vec![false; 600]; 200],

            max_y: 0,
        }
    }

    fn part1(&mut self, _input: &Self::Input) -> String {
        let mut rested_sand = 0;
        let mut path = vec![];

        let mut curr = Pos::new(500, 0);
        let mut didnt_all = true;
        let mut curr_dir = Some(Went::Down);

        while curr.y <= self.max_y {
            if let Some(dir) = curr_dir {
                let followed = curr.follow(dir);
                if misses_block(&self.blocksp1, self.max_y, &followed) {
                    path.push(dir);
                    didnt_all = true;
                    curr = followed;
                    curr_dir = Some(Went::Down);
                } else {
                    curr_dir = dir.next();
                }
            } else {
                if didnt_all {
                    rested_sand += 1;
                    self.blocksp1[curr.index()] = false;
                }
                let last = path.pop().unwrap();
                curr = curr.follow_rev(last);
                curr_dir = Some(last);
                didnt_all = false;
            }
        }
        rested_sand.to_string()
    }

    fn part2(&mut self, _input: &Self::Input) -> String {
        let mut rested_sand = 0;
        let mut current_layer = 1;

        for layer_index in 0..self.max_y + 2 {
            rested_sand += current_layer;
            rested_sand -= self.blocksp2[layer_index].iter().filter(|&&x| x).count();

            for i in 0..self.blocksp2[layer_index].len() - 2 {
                if self.blocksp2[layer_index][i]
                    && self.blocksp2[layer_index][i + 1]
                    && self.blocksp2[layer_index][i + 2]
                {
                    self.blocksp2[layer_index + 1][i + 1] = true;
                }
            }

            current_layer += 2;
        }

        rested_sand.to_string()
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
                    self.blocksp1[Pos::new_index(x, y)] = false;
                    self.blocksp2[y][x] = true;
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
                self.blocksp1[Pos::new_index(x, y)] = false;
                self.blocksp2[y][x] = true;
            }
        }
    }
}
