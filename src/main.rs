mod day_1a;
mod day_1b;
mod day_2a;
mod day_2b;
mod day_3a;
mod day_3b;

use clap::Parser;
use std::error::Error;
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
    Ok(file.map(fs::read_to_string).unwrap_or(Ok(or.to_owned()))?)
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
        gen_views!("1a", "1b", "2a", "2b", "3a", "3b");
        Ok(())
    } else {
        let solution = match args.day.as_ref() {
            "1a" => day_1a::solution(&read_or(args.input, include_str!("./inputs/day_1.txt"))?),
            "1b" => day_1b::solution(&read_or(args.input, include_str!("./inputs/day_1.txt"))?),
            "2a" => day_2a::solution(&read_or(args.input, include_str!("./inputs/day_2.txt"))?),
            "2b" => day_2b::solution(&read_or(args.input, include_str!("./inputs/day_2.txt"))?),
            "3a" => day_3a::solution(&read_or(args.input, include_str!("./inputs/day_3.txt"))?),
            "3b" => day_3b::solution(&read_or(args.input, include_str!("./inputs/day_3.txt"))?),
            _ => {
                eprintln!("the solution to this day isn't here yet!");
                process::exit(1);
            }
        };
        println!("{solution}");
        Ok(())
    }
}
