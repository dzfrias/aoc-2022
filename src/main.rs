mod day_1a;
mod day_1b;
mod day_2a;
mod day_2b;

use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Solution to print to screen
    #[arg(value_parser = valid_day)]
    day: String,

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

fn main() {
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
        gen_views!("1a", "1b", "2a", "2b");
    } else {
        let solution = match args.day.as_ref() {
            "1a" => day_1a::solution(include_str!("./inputs/day_1.txt")),
            "1b" => day_1b::solution(include_str!("./inputs/day_1.txt")),
            "2a" => day_2a::solution(include_str!("./inputs/day_2.txt")),
            "2b" => day_2b::solution(include_str!("./inputs/day_2.txt")),
            _ => {
                eprintln!("the solution to this day isn't here yet!");
                process::exit(1);
            }
        };
        println!("{solution}");
    }
}
