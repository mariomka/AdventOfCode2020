use day18::pest_grammar;
use helpers::{input_lines, run};

fn main() {
    let raw_input = include_str!("../input.txt");
    let input: Vec<&str> = input_lines(raw_input);

    run("part1", || day18::part1::solve(&input));
    run("part2", || day18::part2::solve(&input));

    run("part1 (pest_grammar)", || {
        pest_grammar::solve(raw_input, false)
    });
    run("part2 (pest_grammar)", || {
        pest_grammar::solve(raw_input, true)
    });
}
