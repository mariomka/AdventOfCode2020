use helpers::{parse_input, run};

fn main() {
    let input: Vec<usize> = parse_input(include_str!("../input.txt"));

    run("part1", || day25::part1(&input));
}
