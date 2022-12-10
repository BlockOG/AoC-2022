use crate::days;

fn get_priority(c: char) -> u32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,

        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 53,
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = Vec<String>;

    fn get_num(&self) -> u8 {
        3
    }

    fn part1(&self, input: &Self::Input) -> String {
        let rucksacks = input
            .iter()
            .map(|s| {
                let split = s.split_at(s.len() / 2);
                (split.0.to_string(), split.1.to_string())
            })
            .collect::<Vec<(String, String)>>();
        let mut priority = 0;
        for (a, b) in rucksacks.iter() {
            for ca in a.chars() {
                if b.contains(ca) {
                    priority += get_priority(ca);
                    break;
                }
            }
        }
        priority.to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let rucksack_groups = input.chunks(3).collect::<Vec<&[String]>>();
        let mut priority = 0;
        for group in rucksack_groups {
            for ca in group[0].chars() {
                if group[1].contains(ca) && group[2].contains(ca) {
                    priority += get_priority(ca);
                    break;
                }
            }
        }
        priority.to_string()
    }

    fn parse_input(&self, input: &String) -> Self::Input {
        input.lines().map(|s| s.to_string()).collect()
    }
}
