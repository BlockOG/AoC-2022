use colored::*;
use std::{
    env::args,
    io::{stdin, stdout, Write},
};

use crate::days::run_day;

mod days;

fn main() {
    let args: Vec<String> = args().collect();
    let day_num = match args.get(1) {
        Some(day_num) => {
            if day_num == "all" {
                for day in 1..=25 {
                    run_day(day);
                }
                return;
            }
            match day_num.parse::<u8>() {
                Ok(day_num) => day_num,
                Err(_) => {
                    println!("{}", "Please provide a valid day number or `all`".bold().red());
                    return;
                }
            }
        }
        None => {
            print!("No day provided. Run all days? ({}/{}) ", "y".bold().green(), "n".bold().red());
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            if input.trim() == "y" {
                for day in 1..=25 {
                    run_day(day);
                }
            }
            return;
        }
    };

    run_day(day_num);
}
