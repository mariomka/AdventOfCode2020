use helpers::{parse_input, run};
use std::collections::HashSet;

fn main() {
    let input: HashSet<u64> = parse_input(include_str!("../input.txt"));

    run("part1", || day1::part1(&input));
    run("part2", || day1::part2(&input));
}
