use clap::{arg, command};
use colored::*;

use crate::days::{nanos_to_string, run_day};

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
        .arg(arg!(
            -p --dontprint "Don't print the output of the days"
        ))
        .arg(arg!(
            -s --dontsubmit "Don't auto-submit the answer"
        ))
        .arg(arg!(
            -i --dontinput "Don't auto-get the input"
        ))
        .get_matches();

    let day = matches.get_one::<String>("DAY").unwrap();
    let time = *matches.get_one::<bool>("time").unwrap();
    let dont_print = *matches.get_one::<bool>("dontprint").unwrap();
    let dontsubmit = *matches.get_one::<bool>("dontsubmit").unwrap();
    let dontinput = *matches.get_one::<bool>("dontinput").unwrap();
    let client = reqwest::blocking::Client::new();
    if day == "all" {
        let mut total_time = (0, 0, 0);
        for day in 1..=25 {
            let took_time = run_day(day, time, dont_print, dontsubmit, dontinput, &client);
            if let Some((parsing, part1, part2)) = took_time {
                total_time.0 += parsing;
                total_time.1 += part1;
                total_time.2 += part2;
            }
        }
        if *matches.get_one::<bool>("totaltime").unwrap() {
            println!(
                "{} {}",
                "Total Parsing Time:".bold(),
                nanos_to_string(total_time.0)
            );
            println!(
                "{} {}",
                "Total Part 1 Time:".bold(),
                nanos_to_string(total_time.1)
            );
            println!(
                "{} {}",
                "Total Part 2 Time:".bold(),
                nanos_to_string(total_time.2)
            );
            println!(
                "{} {}",
                "Total Parts Time:".bold(),
                nanos_to_string(total_time.1 + total_time.2)
            );
            println!(
                "{} {}",
                "Total Time:".bold(),
                nanos_to_string(total_time.0 + total_time.1 + total_time.2)
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
        dont_print,
        dontsubmit,
        dontinput,
        &client,
    );
    if let Some((parsing, part1, part2)) = total_time {
        if *matches.get_one::<bool>("totaltime").unwrap() {
            println!(
                "{} {}",
                "Total Parts Time:".bold(),
                nanos_to_string(part1 + part2)
            );
            println!(
                "{} {}",
                "Total Time:".bold(),
                nanos_to_string(parsing + part1 + part2)
            );
        }
    }
}
