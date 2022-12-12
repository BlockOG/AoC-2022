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
        .get_matches();

    let day = matches.get_one::<String>("DAY").unwrap();
    let time = *matches.get_one::<bool>("time").unwrap();
    if day == "all" {
        for day in 1..=25 {
            run_day(day, time);
        }
        return;
    }
    run_day(
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
    );
}
