use colored::*;
use paste::paste;
use std::{fs::File, io::Read, time::Instant};

macro_rules! run_days {
    ($the_day:expr, $the_time:expr, $the_dont_print:expr, $($day:expr),+) => {
        $(
            paste! { mod [<day $day>] {
                include!(concat!(stringify!([<day $day>]), ".rs"));
            } }
        )+

        return match $the_day {
            $(
                $day => run_impled_day(paste! { &mut [<day $day>]::Day::new() }, $the_time, $the_dont_print),
            )+
            _ => {
                println!("{}", format!("Day {} not implemented yet", $the_day).bold().red());
                None
            }
        }
    }
}

pub trait Day {
    type Input;

    fn get_num(&self) -> u8;
    fn new() -> Self;
    fn part1(&mut self, input: &Self::Input) -> String;
    fn part2(&mut self, input: &Self::Input) -> String;
    fn parse_input(&mut self, input: &String) -> Self::Input;
}

fn nanos_to_most_convenient(nanos: u128) -> (f64, String) {
    let mut nanos = nanos as f64;
    let mut unit = "ns";
    if nanos > 1_000_000_000f64 {
        nanos /= 1_000_000_000f64;
        unit = "s";
    } else if nanos > 1_000_000f64 {
        nanos /= 1_000_000f64;
        unit = "ms";
    } else if nanos > 1_000f64 {
        nanos /= 1_000f64;
        unit = "us";
    }
    (nanos, unit.to_string())
}

pub fn nanos_to_string(nanos: u128) -> String {
    let (num, unit) = nanos_to_most_convenient(nanos);
    format!("{} {}", num, unit)
}

fn run_impled_day(day: &mut impl Day, time: bool, dont_print: bool) -> Option<(u128, u128, u128)> {
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
                return None;
            }
        },
        Err(_) => {
            println!(
                "{}",
                format!("Could not open input file ({})", input_file_path)
                    .bold()
                    .red()
            );
            return None;
        }
    }
    input = input.replace("\r\n", "\n");

    let start_parsing = Instant::now();
    let parsed_input = day.parse_input(&input);
    let elapsed_parsing = start_parsing.elapsed().as_nanos();

    println!("{}", format!("Day {}", day.get_num()).bold().green());

    let start_part1 = Instant::now();
    let part1 = day.part1(&parsed_input);
    let elapsed_part1 = start_part1.elapsed().as_nanos();
    if !dont_print {
        println!("{} {}", "Part 1:".bold(), part1);
    }

    let start_part2 = Instant::now();
    let part2 = day.part2(&parsed_input);
    let elapsed_part2 = start_part2.elapsed().as_nanos();
    if !dont_print {
        println!("{} {}", "Part 2:".bold(), part2);
    }

    if time {
        println!(
            "{} {}",
            "Parsing Time:".bold(),
            nanos_to_string(elapsed_parsing)
        );
        println!(
            "{} {}",
            "Part 1 Time:".bold(),
            nanos_to_string(elapsed_part1)
        );
        println!(
            "{} {}",
            "Part 2 Time:".bold(),
            nanos_to_string(elapsed_part2)
        );
    }
    Some((elapsed_parsing, elapsed_part1, elapsed_part2))
}

pub fn run_day(day_num: u8, time: bool, dont_print: bool) -> Option<(u128, u128, u128)> {
    if day_num < 1 || day_num > 25 {
        println!("{}", "Day number must be between 1 and 25".bold().red());
        return None;
    }
    run_days!(day_num, time, dont_print, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
}
