use helpers::{run, split_input};

fn main() {
    let input: Vec<&str> = split_input(include_str!("../input.txt"), "\n\n");

    run("part1", || day6::part1(&input));
    run("part2", || day6::part2(&input));
}
