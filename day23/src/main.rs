use helpers::{parse_split_input, run};

fn main() {
    let input: Vec<usize> = parse_split_input(include_str!("../input.txt"), "");

    run("part1", || day23::part1(&input));
    run("part2", || day23::part2(&input));
}
