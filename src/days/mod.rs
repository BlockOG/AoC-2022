use colored::*;
use paste::paste;
use std::{fs::File, io::Read};

macro_rules! run_days {
    ($the_day:expr, $($day:expr),+) => {
        $(
            paste! { mod [<day $day>] {
                include!(concat!(stringify!([<day $day>]), ".rs"));
            } }
        )+

        match $the_day {
            $(
                $day => run_impled_day(paste! { &[<day $day>]::Day {} }),
            )+
            _ => println!("{}", format!("Day {} not implemented yet", $the_day).bold().red())
        }
    }
}

pub trait Day {
    type Input;

    fn get_num(&self) -> u8;
    fn part1(&self, input: &Self::Input) -> String;
    fn part2(&self, input: &Self::Input) -> String;
    fn parse_input(&self, input: &String) -> Self::Input;
}

fn run_impled_day(day: &impl Day) {
    let mut input = String::new();
    let input_file_path = format!("inputs/input{}.txt", day.get_num());
    match File::open(&input_file_path) {
        Ok(mut file) => match file.read_to_string(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("{}", format!("Could not read input file ({})", input_file_path).bold().red());
                return;
            }
        },
        Err(_) => {
            println!("{}", format!("Could not open input file ({})", input_file_path).bold().red());
            return;
        }
    }
    input = input.replace("\r\n", "\n");

    let parsed_input = day.parse_input(&input);
    println!("{}", format!("Day {}", day.get_num()).bold().green());
    println!("{} {}", "Part 1:".bold(), day.part1(&parsed_input));
    println!("{} {}", "Part 2:".bold(), day.part2(&parsed_input));
}

pub fn run_day(day_num: u8) {
    if day_num < 1 || day_num > 25 {
        println!("{}", "Day number must be between 1 and 25".bold().red());
        return;
    }
    run_days!(day_num, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
}
