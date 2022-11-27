# Advent of Code Template

This is a template for solving the [advent of code](https://adventofcode.com) 
challenges in rust. This template makes it easy to keep your solutions organized,
and gets rid of the setup. This way you can focus on solving the puzzles, not the
setup.

## Features

 - CLI Arg parsing with [clap](https://docs.rs/clap) to easily configure what runs
   - By default, `cargo run` will run your most recent day, which is usually what you 
want.
   - You can run a specific day with `cargo r -- -d4` (replacing 4 with the day you want to run).
or run all of them with `cargo run -- -a`.
 - Solutions are automatically timed so that you have a general idea of execution 
time. Of course, make sure to run with the `--release` flag before making any 
optimizations.
 - Pretty output: This template makes use of the [colored](https://docs.rs/colored)
crate to color and bold output. The results of your code are colored according to
`solutions.rs`: blue if you haven't entered a solution yet, green if they match,
or red if they don't match. 
 - Tests: If you put your solutions in `solutions.rs`, running `cargo test` will 
 test the current day. This can be helpful if you want to refactor your solution
 after you solved it. To test every day, run `cargo test -- --include-ignored`.

## Get Started

1. On GitHub, click "Use this template" then "Create New Repository".
2. Download your input, and put it in the `./input/` folder with the format
`dXX.txt`. ex. `d05.txt`.
3. Write your day 1 code in `./src/days/d1.rs`. 
4. As you work though the puzzles, increment the `CURRENT_DAY` constant in 
`main.rs`, so that `cargo run` runs the right puzzle.
5. When you solve a puzzle, put your solution in `solutions.rs`. This colors
your output green or red according to if the solution matches or not, and 
allows you to do `cargo test` to test your solutions.

## License

This template is licensed under the [CC0 1.0 Universal](LICENSE).

## Contributions

If you see something that could use improvement, or you want to add a feature, 
open a PR. Any contributions made will be subject to the above license.
