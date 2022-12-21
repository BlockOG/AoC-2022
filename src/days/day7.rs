use std::collections::HashMap;

use crate::days;

fn get_size(
    search_dir: String,
    input: &HashMap<String, Vec<(u32, String)>>,
    sizes: &mut HashMap<String, u32>,
) -> u32 {
    let mut size = 0;
    for (num, dir) in input.get(&search_dir).unwrap() {
        if num == &0 {
            let mut search_dir = search_dir.clone();
            if !search_dir.ends_with("/") {
                search_dir += "/";
            }
            size += get_size(search_dir + dir, input, sizes);
        } else {
            size += num;
        }
    }
    sizes.insert(search_dir, size);
    size
}

pub struct Day {
    day_num: u8,
}

impl days::Day for Day {
    type Input = HashMap<String, Vec<(u32, String)>>;

    fn get_num(&self) -> u8 {
        self.day_num
    }

    fn new(day_num: u8) -> Self {
        Self {
            day_num
        }
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        let mut sizes = HashMap::new();
        get_size("/".to_string(), input, &mut sizes);
        (
            sizes
                .values()
                .filter(|&&size| size < 100000)
                .sum::<u32>()
                .to_string(),
            true,
        )
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        let mut sizes = HashMap::new();
        let delete_size = 30000000 - (70000000 - get_size("/".to_string(), input, &mut sizes));
        (
            sizes
                .iter()
                .map(|x| x.1)
                .filter(|&&x| x >= delete_size)
                .min()
                .unwrap()
                .to_string(),
            true,
        )
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        let mut map: Self::Input = HashMap::new();
        let mut curr_dir = String::new();
        for line in input.lines() {
            if line.chars().nth(0).unwrap() == '$' {
                let mut words = line.split_whitespace();
                words.next();
                let command = words.next().unwrap();

                if command == "ls" {
                    // map.insert(curr_dir.clone(), Vec::new());
                } else if command == "cd" {
                    let dir = words.next().unwrap();
                    if dir == ".." {
                        curr_dir = curr_dir
                            .split('/')
                            .take(curr_dir.split('/').count() - 1)
                            .collect::<Vec<&str>>()
                            .join("/");
                    } else {
                        if !curr_dir.ends_with("/") && !dir.starts_with("/") {
                            curr_dir += "/";
                        }
                        curr_dir += &dir.to_string();
                    }
                }
            } else {
                let mut words = line.split_whitespace();
                let num = match words.next().unwrap() {
                    "dir" => 0,
                    num => num.parse::<u32>().unwrap(),
                };
                let dir = words.next().unwrap();
                if !map.contains_key(&curr_dir) {
                    map.insert(curr_dir.clone(), Vec::new());
                }
                map.get_mut(&curr_dir).unwrap().push((num, dir.to_string()));
            }
        }
        map
    }
}
