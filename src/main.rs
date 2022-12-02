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
        match args.day.as_ref() {
            "1a" => println!("{}", include_str!("./day_1a/mod.rs")),
            "1b" => println!("{}", include_str!("./day_1b/mod.rs")),
            "2a" => println!("{}", include_str!("./day_2a/mod.rs")),
            "2b" => println!("{}", include_str!("./day_2b/mod.rs")),
            _ => {
                eprintln!("the solution to this day isn't here yet!");
                process::exit(1);
            }
        }
    } else {
        let solution = match args.day.as_ref() {
            "1a" => day_1a::solution(),
            "1b" => day_1b::solution(),
            "2a" => day_2a::solution(),
            "2b" => day_2b::solution(),
            _ => {
                eprintln!("the solution to this day isn't here yet!");
                process::exit(1);
            }
        };
        println!("{solution}");
    }
}
