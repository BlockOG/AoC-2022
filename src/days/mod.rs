use std::collections::HashMap;
use std::io::Write;
use std::{fs::File, io::Read, time::Instant};

use colored::*;
use paste::paste;
use regex::Regex;

macro_rules! run_day {
    ($($day:expr),+) => {
        $(
            paste! { mod [<day $day>] ; }
        )+

        pub fn run_day(day_num: u8, time: bool, dont_print: bool, dontsubmit: bool, dontinput: bool, client: &reqwest::blocking::Client) -> Option<(u128, u128, u128)> {
            if day_num < 1 || day_num > 25 {
                println!("{}", "Day number must be between 1 and 25".bold().red());
                return None;
            }
            return match day_num {
                $(
                    $day => run_impled_day(paste! { &mut [<day $day>]::Day::new(day_num) }, time, dont_print, dontsubmit, dontinput, client),
                )+
                _ => {
                    println!("{}", format!("Day {} not implemented yet", day_num).bold().red());
                    None
                }
            }
        }
    }
}

pub trait Day {
    type Input;

    fn get_num(&self) -> u8;
    fn new(day_num: u8) -> Self;
    fn part1(&mut self, input: &Self::Input) -> (String, bool);
    fn part2(&mut self, input: &Self::Input) -> (String, bool);
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

pub fn nanos_to_string(nanos: u128) -> ColoredString {
    let (num, unit) = nanos_to_most_convenient(nanos);
    if nanos < 500_000_000 {
        format!("{} {}", num, unit).white()
    } else {
        format!("{} {}", num, unit).red()
    }
}

fn submit(
    day: &mut impl Day,
    level: u8,
    answer: &String,
    session: &String,
    client: &reqwest::blocking::Client,
) -> bool {
    let mut form = HashMap::new();
    form.insert("level", level.to_string());
    form.insert("answer", answer.to_string());

    match client
        .post(format!(
            "https://adventofcode.com/2022/day/{}/answer",
            day.get_num()
        ))
        .header("Cookie", format!("session={}", session))
        .header(
            "User-Agent",
            "BlockOG's AoC 2022 solutions at https://github.com/BlockOG/AoC2022",
        )
        .form(&form)
        .send()
    {
        Ok(resp) => {
            let resp = resp.text().unwrap();
            let mut file =
                File::create(format!("logs/submit{}_{}.txt", day.get_num(), level)).unwrap();
            file.write_all(resp.as_bytes()).unwrap();
            if resp.contains("one gold star") {
                println!("{}", "Answer correct!".bold().green());
                return true;
            } else {
                println!("{}", "Answer incorrect :(".bold().red());
                return false;
            }
        }
        Err(_) => {
            println!("{}", "Could not submit answer".bold().red());
            return false;
        }
    };
}

fn run_impled_day(
    day: &mut impl Day,
    time: bool,
    dont_print: bool,
    dont_submit: bool,
    dont_input: bool,
    client: &reqwest::blocking::Client,
) -> Option<(u128, u128, u128)> {
    let mut session = String::new();
    if !dont_submit || !dont_input {
        match File::open("inputs/session.txt") {
            Ok(mut file) => match file.read_to_string(&mut session) {
                Ok(_) => (),
                Err(_) => {
                    println!(
                        "{}",
                        "Could not read session file (inputs/session.txt)"
                            .bold()
                            .red()
                    );
                    return None;
                }
            },
            Err(_) => {
                println!(
                    "{}",
                    "Could not open session file (inputs/session.txt)"
                        .bold()
                        .red()
                );
                return None;
            }
        }
        session = session.trim().to_string();
    }

    let mut completed = 0;
    let mut part1_answer = String::new();
    let mut part2_answer = String::new();
    let answer_regex = Regex::new(r"<p>Your puzzle answer was <code>(\w+)</code>\.</p>").unwrap();
    if !dont_submit {
        match client
            .get(&format!(
                "https://adventofcode.com/2022/day/{}",
                day.get_num()
            ))
            .header("Cookie", format!("session={}", session))
            .header(
                "User-Agent",
                "BlockOG's AoC 2022 solutions at https://github.com/BlockOG/AoC2022",
            )
            .send()
        {
            Ok(response) => match response.text() {
                Ok(text) => {
                    let mut matches = answer_regex.captures_iter(&text);
                    if let Some(match1) = matches.next() {
                        part1_answer = match1[1].to_string();
                        completed += 1;
                    }
                    if let Some(match2) = matches.next() {
                        part2_answer = match2[1].to_string();
                        completed += 1;
                    }
                }
                Err(_) => {
                    println!("{}", "Could not get star amount".bold().red());
                    return None;
                }
            },
            Err(err) => {
                println!("{}", "Could not download star amount".bold().red());
                println!("{}", err);
                return None;
            }
        };
    }

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
            if dont_input {
                println!(
                    "{}",
                    format!("Could not open input file ({})", input_file_path)
                        .bold()
                        .red()
                );
                return None;
            } else {
                println!("{}", "Downloading input...".bold());
                input = match client
                    .get(&format!(
                        "https://adventofcode.com/2022/day/{}/input",
                        day.get_num()
                    ))
                    .header("Cookie", format!("session={}", session))
                    .header(
                        "User-Agent",
                        "BlockOG's AoC 2022 solutions at https://github.com/BlockOG/AoC2022",
                    )
                    .send()
                {
                    Ok(response) => match response.text() {
                        Ok(text) => text.trim_end().to_string(),
                        Err(_) => {
                            println!("{}", "Could not read input".bold().red());
                            return None;
                        }
                    },
                    Err(err) => {
                        println!("{}", "Could not download input".bold().red());
                        println!("{}", err);
                        return None;
                    }
                };
                println!("{}", "Input downloaded".bold());
                match File::create(&input_file_path) {
                    Ok(mut file) => match file.write_all(input.as_bytes()) {
                        Ok(_) => (),
                        Err(_) => {
                            println!(
                                "{}",
                                format!("Could not write input file ({})", input_file_path)
                                    .bold()
                                    .red()
                            );
                            return None;
                        }
                    },
                    Err(_) => {
                        println!(
                            "{}",
                            format!("Could not create input file ({})", input_file_path)
                                .bold()
                                .red()
                        );
                        return None;
                    }
                }
            }
        }
    }
    input = input.replace("\r\n", "\n");

    let start_parsing = Instant::now();
    let parsed_input = day.parse_input(&input);
    let elapsed_parsing = start_parsing.elapsed().as_nanos();

    if completed > 0 {
        println!(
            "{} {}",
            format!("Day {}", day.get_num()).bold().green(),
            "*".repeat(completed).bold().yellow()
        );
    } else {
        println!("{}", format!("Day {}", day.get_num()).bold().green());
    }

    let start_part1 = Instant::now();
    let part1 = day.part1(&parsed_input);
    let elapsed_part1 = start_part1.elapsed().as_nanos();
    if !dont_print {
        if part1_answer.is_empty() || !part1.1 {
            println!("{} {}", "Part 1:".bold(), part1.0);
        } else {
            if part1.0 == part1_answer {
                println!("{} {}", "Part 1:".bold(), part1.0.green());
            } else {
                println!("{} {}", "Part 1:".bold(), part1.0.red());
                println!("        {}", part1_answer.green());
            }
        }
    }
    let mut failed_submission = false;
    if !dont_submit && completed < 1 {
        println!("{}", "Submitting part 1...".bold());
        failed_submission = !submit(day, 1, &part1.0, &session, client);
    }

    let start_part2 = Instant::now();
    let part2 = day.part2(&parsed_input);
    let elapsed_part2 = start_part2.elapsed().as_nanos();
    if !dont_print {
        if part2_answer.is_empty() || !part2.1 {
            println!("{} {}", "Part 2:".bold(), part2.0);
        } else {
            if part2.0 == part2_answer {
                println!("{} {}", "Part 2:".bold(), part2.0.green());
            } else {
                println!("{} {}", "Part 2:".bold(), part2.0.red());
                println!("        {}", part2_answer.green());
            }
        }
    }
    if !dont_submit && completed < 2 && !failed_submission {
        println!("{}", "Submitting part 2...".bold());
        if submit(day, 2, &part2.0, &session, client) {
            println!("{}", "Day completed!".bold().green());
        }
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

run_day!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 23);
