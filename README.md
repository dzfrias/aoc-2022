# Advent of Code 2022
These are my [Advent of Code](https://adventofcode.com/) solutions for 2022!
I chose [Rust](https://github.com/rust-lang/rust) for this year. Some days I
went for speed, and others I took a more robust approach.

## Layout
Inside the [src](/src) directory, there are folders for the days I have
completed. My solutions will be in those respective directories. In each
solution file there is a test case, which holds the samples that Advent of Code
gives.

The [inputs](/src/inputs) directory has every input I used for each day.

I have a [main.rs](/src/main.rs) because this repository doubles as a CLI app!

## CLI
I coupled my solutions with a command line application, which allows me to run
each solution through any arbitrary input. It also allows me to easily print
the code for the solution to the screen.

### Getting Started
To get started, run:
```
$ aoc-2022 1a
```
This should put **my** solution on the screen. Because Advent of Code inputs are
different for each person, this will **not** be the solution for everybody.

### Arbitrary Input
To run it with any arbitrary input, run
```
$ aoc-2022 1a some_file.txt
```
This will read `some_file.txt` and run the solution on that input. Note that
the program will likely crash if the input is malformed!

### Viewing
Finally, if you'd just like to view the solution's code, run
```
$ aoc-2022 1a --view
```
You can also pass in `-v` instead of `--view`.

## Installation
This project is **not** on [crates.io](https://crates.io/), so `cargo install`
won't work.

To install, clone this project manually and build it with the Rust. Make sure to
build with the `--release` flag for optimal performance.
