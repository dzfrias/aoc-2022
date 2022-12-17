mod day_10a;
mod day_10b;
mod day_11a;
mod day_13a;
mod day_13b;
mod day_14a;
mod day_14b;
mod day_1a;
mod day_1b;
mod day_2a;
mod day_2b;
mod day_3a;
mod day_3b;
mod day_4a;
mod day_4b;
mod day_5a;
mod day_5b;
mod day_6a;
mod day_6b;
mod day_8a;
mod day_8b;
mod day_9a;
mod day_9b;

use clap::Parser;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Solution to print to screen
    #[arg(value_parser = valid_day)]
    day: String,

    /// File to use as input
    #[arg(value_name = "FILE", conflicts_with = "view")]
    input: Option<PathBuf>,

    /// View the solution of a given day
    #[arg(short, long)]
    view: bool,
}

fn valid_day(day: &str) -> Result<String, String> {
    if !matches!(
        day.chars()
            .last()
            .expect("should have at least one character"),
        'a' | 'b'
    ) {
        return Err(format!(
            "Day `{day}` should have 'a' or 'b' as its last character"
        ));
    }
    let day_num = day
        .chars()
        .take(day.len() - 1)
        .collect::<String>()
        .parse::<i32>();
    match day_num {
        Ok(num) if num > 31 => Err(format!("Day `{num}` should be under 32")),
        Err(_) => Err(format!("Day `{day}` should be a valid integer")),
        _ => Ok(day.to_owned()),
    }
}

fn read_or(file: Option<PathBuf>, or: &str) -> Result<String, Box<dyn Error>> {
    Ok(file
        .map(fs::read_to_string)
        .unwrap_or_else(|| Ok(or.to_owned()))?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if args.view {
        macro_rules! gen_views {
            ($($day:tt),+) => {
                match args.day.as_ref() {
                    $($day => println!("{}", include_str!(concat!("./day_", $day, "/mod.rs"))),)+
                    _ => {
                        eprintln!("the solution to this day isn't here yet!");
                        process::exit(1);
                    }
                }
            };
        }
        gen_views!(
            "1a", "1b", "2a", "2b", "3a", "3b", "4a", "4b", "5a", "5b", "6a", "6b", "8a", "8b",
            "9a", "9b", "10a", "10b", "13a", "13b"
        );
        Ok(())
    } else {
        let solution: Box<dyn Display> = match args.day.as_ref() {
            "1a" => Box::new(day_1a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_1.txt"),
            )?)),
            "1b" => Box::new(day_1b::solution(&read_or(
                args.input,
                include_str!("./inputs/day_1.txt"),
            )?)),
            "2a" => Box::new(day_2a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_2.txt"),
            )?)),
            "2b" => Box::new(day_2b::solution(&read_or(
                args.input,
                include_str!("./inputs/day_2.txt"),
            )?)),
            "3a" => Box::new(day_3a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_3.txt"),
            )?)),
            "3b" => Box::new(day_3b::solution(&read_or(
                args.input,
                include_str!("./inputs/day_3.txt"),
            )?)),
            "4a" => Box::new(day_4a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_4.txt"),
            )?)),
            "4b" => Box::new(day_4b::solution(&read_or(
                args.input,
                include_str!("./inputs/day_4.txt"),
            )?)),
            "5a" => Box::new(day_5a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_5.txt"),
            )?)),
            "5b" => Box::new(day_5b::solution(&read_or(
                args.input,
                include_str!("./inputs/day_5.txt"),
            )?)),
            "6a" => Box::new(day_6a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_6.txt"),
            )?)),
            "6b" => Box::new(day_6b::solution(&read_or(
                args.input,
                include_str!("./inputs/day_6.txt"),
            )?)),
            "8a" => Box::new(day_8a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_8.txt"),
            )?)),
            "8b" => Box::new(day_8b::solution(&read_or(
                args.input,
                include_str!("./inputs/day_8.txt"),
            )?)),
            "9a" => Box::new(day_9a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_9.txt"),
            )?)),
            "9b" => Box::new(day_9b::solution(&read_or(
                args.input,
                include_str!("./inputs/day_9.txt"),
            )?)),
            "10a" => Box::new(day_10a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_10.txt"),
            )?)),
            "10b" => Box::new(day_10b::solution(&read_or(
                args.input,
                include_str!("./inputs/day_10.txt"),
            )?)),
            "11a" => Box::new(day_11a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_11.txt"),
            )?)),
            "13a" => Box::new(day_13a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_13.txt"),
            )?)),
            "13b" => Box::new(day_13b::solution(&read_or(
                args.input,
                include_str!("./inputs/day_13.txt"),
            )?)),
            "14a" => Box::new(day_14a::solution(&read_or(
                args.input,
                include_str!("./inputs/day_14.txt"),
            )?)),
            "14b" => Box::new(day_14b::solution(&read_or(
                args.input,
                include_str!("./inputs/day_14.txt"),
            )?)),
            _ => {
                eprintln!("the solution to this day isn't here yet!");
                process::exit(1);
            }
        };
        println!("{solution}");
        Ok(())
    }
}
