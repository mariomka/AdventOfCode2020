use helpers::{input_lines, run};

fn main() {
    let input: Vec<&str> = input_lines(include_str!("../input.txt"));

    run("part1", &input, day2::part1);
    run("part2", &input, day2::part2);
}
