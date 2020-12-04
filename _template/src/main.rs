use helpers::{input_lines, run};

fn main() {
    let input: Vec<&str> = input_lines(include_str!("../input.txt"));

    run("part1", &input, || dayN::part1(&input));
    run("part2", &input, || dayN::part1(&input));
}
