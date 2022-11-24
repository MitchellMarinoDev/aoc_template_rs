# Advent of Code Template

This is a template for solving the [advent of code](https://adventofcode.com) 
challenges in rust. This template makes it easy to keep your solutions organized,
and gets rid of the setup. This way you can focus on solving the puzzles, not the
setup.

## Features

 - CLI Arg parsing with [clap](https://docs.rs/clap) to easily configure what runs
   - By default, `cargo run` will run your most recent day which is usually what you 
want
   - You can run a specific day with `cargo r -- -d4`, or run all of them with 
     `cargo run -- -a`.
 - Solutions are automatically timed so that you have a general idea of execution 
time. Of course, make sure to run with the `--release` flag before making any 
optimizations.
 - Pretty output: This template makes use of the [colored](https://docs.rs/colored)
crate to color and bold output.

### TODO:

 - [ ] Add tests and benches for the current day and all days.
 - [ ] Add a place to put the answers once you have solved the puzzles. This will
 be used for the tests.

## Get Started

Clone the repo with 
`git clone https://github.com/MitchellMarinoDev/aoc_template_rs.git`.
First, download your input, and put it in the `./input/` folder. Then, just write
your day 1 code in `./src/days/d1.rs`. As you work though the puzzles, increment 
the `CURRENT_DAY` constant in `main.rs`, so that `cargo run` runs the right puzzle.

## License

This template is licensed under the [CC0 1.0 Universal](LICENSE).

## Contributions

If you see something that could use improvement, or you want to add a feature, 
open a PR. Any contributions made will be subject to the above license.