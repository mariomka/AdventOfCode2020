use helpers::{parse_input, run};

fn main() {
    let input: Vec<u64> = parse_input(include_str!("../input.txt"));

    run("part1", || day9::part1(&input, 25));
    run("part2", || day9::part2(&input, 25));
}
