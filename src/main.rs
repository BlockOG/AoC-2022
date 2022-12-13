use clap::{arg, command};
use colored::*;

use crate::days::run_day;

mod days;

fn main() {
    let matches = command!()
        .arg(arg!(
            <DAY> "The day to run"
        ))
        .arg(
            arg!(
                -t --time "Time the execution of days"
            )
            .required(false),
        )
        .arg(
            arg!(
                -d --totaltime "Time the total execution of days"
            )
            .required(false),
        )
        .arg(
            arg!(
                -p --dontprint "Don't print the output of the days"
            )
        )
        .get_matches();

    let day = matches.get_one::<String>("DAY").unwrap();
    let time = *matches.get_one::<bool>("time").unwrap();
    let dont_print = *matches.get_one::<bool>("dontprint").unwrap();
    if day == "all" {
        let mut total_time = (0, 0, 0);
        for day in 1..=25 {
            let took_time = run_day(day, time, dont_print);
            if let Some((parsing, part1, part2)) = took_time {
                total_time.0 += parsing;
                total_time.1 += part1;
                total_time.2 += part2;
            }
        }
        if *matches.get_one::<bool>("totaltime").unwrap() {
            println!(
                "{} {} ms",
                "Total Time Parsing:".bold(),
                total_time.0 as f64 / 1_000_000f64
            );
            println!(
                "{} {} ms",
                "Total Time Part 1:".bold(),
                total_time.1 as f64 / 1_000_000f64
            );
            println!(
                "{} {} ms",
                "Total Time Part 2:".bold(),
                total_time.2 as f64 / 1_000_000f64
            );
            println!(
                "{} {} ms",
                "Total Parts Time:".bold(),
                (total_time.1 + total_time.2) as f64 / 1_000_000f64
            );
            println!(
                "{} {} ms",
                "Total Time:".bold(),
                (total_time.0 + total_time.1 + total_time.2) as f64 / 1_000_000f64
            );
        }
        return;
    }
    let total_time = run_day(
        match day.parse::<u8>() {
            Ok(day) => day,
            Err(_) => {
                println!(
                    "{}",
                    "Please provide a valid day number or `all`".bold().red()
                );
                return;
            }
        },
        time,
        dont_print
    );
    if let Some((parsing, part1, part2)) = total_time {
        if *matches.get_one::<bool>("totaltime").unwrap() {
            println!(
                "{} {} ms",
                "Total Parts Time:".bold(),
                (part1 + part2) as f64 / 1_000_000f64
            );
            println!(
                "{} {} ms",
                "Total Time:".bold(),
                (parsing + part1 + part2) as f64 / 1_000_000f64
            );
        }
    }
}
