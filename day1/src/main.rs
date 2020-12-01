use helpers::{parse_input, run};
use std::collections::HashSet;

fn main() {
    let input: HashSet<u64> = parse_input(include_str!("../input.txt"));

    run("part1", &input, day1::part1);
    run("part2", &input, day1::part2);
}
