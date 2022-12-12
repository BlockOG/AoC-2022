mod day12;

use colored::*;
use paste::paste;
use std::{fs::File, io::Read, time::Instant};

macro_rules! run_days {
    ($the_day:expr, $the_time:expr, $($day:expr),+) => {
        $(
            paste! { mod [<day $day>] {
                include!(concat!(stringify!([<day $day>]), ".rs"));
            } }
        )+

        match $the_day {
            $(
                $day => run_impled_day(paste! { &[<day $day>]::Day {} }, $the_time),
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

fn run_impled_day(day: &impl Day, time: bool) {
    let mut input = String::new();
    let input_file_path = format!("inputs/input{}.txt", day.get_num());
    match File::open(&input_file_path) {
        Ok(mut file) => match file.read_to_string(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!(
                    "{}",
                    format!("Could not read input file ({})", input_file_path)
                        .bold()
                        .red()
                );
                return;
            }
        },
        Err(_) => {
            println!(
                "{}",
                format!("Could not open input file ({})", input_file_path)
                    .bold()
                    .red()
            );
            return;
        }
    }
    input = input.replace("\r\n", "\n");

    let start_parsing = Instant::now();
    let parsed_input = day.parse_input(&input);
    let elapsed_parsing = start_parsing.elapsed().as_nanos();

    println!("{}", format!("Day {}", day.get_num()).bold().green());

    let start_part1 = Instant::now();
    println!("{} {}", "Part 1:".bold(), day.part1(&parsed_input));
    let elapsed_part1 = start_part1.elapsed().as_nanos();

    let start_part2 = Instant::now();
    println!("{} {}", "Part 2:".bold(), day.part2(&parsed_input));
    let elapsed_part2 = start_part2.elapsed().as_nanos();

    if !time {
        return;
    }
    println!(
        "{} {} ms",
        "Time Parsing:".bold(),
        elapsed_parsing as f64 / 1_000_000f64
    );
    println!(
        "{} {} ms",
        "Time Part 1:".bold(),
        elapsed_part1 as f64 / 1_000_000f64
    );
    println!(
        "{} {} ms",
        "Time Part 2:".bold(),
        elapsed_part2 as f64 / 1_000_000f64
    );
}

pub fn run_day(day_num: u8, time: bool) {
    if day_num < 1 || day_num > 25 {
        println!("{}", "Day number must be between 1 and 25".bold().red());
        return;
    }
    run_days!(day_num, time, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
}
