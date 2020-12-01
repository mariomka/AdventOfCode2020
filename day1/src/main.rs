use helpers::{parse_input_hashset, run};
use std::collections::HashSet;

fn main() {
    let input: HashSet<u64> = parse_input_hashset(include_str!("../input.txt"));

    run("part1", &input, day1::part1);
    run("part2", &input, day1::part2);
}
