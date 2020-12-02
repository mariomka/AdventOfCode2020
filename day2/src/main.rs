use helpers::{parse_input, run};

fn main() {
    let input: Vec<String> = parse_input(include_str!("../input.txt"));

    run("part1", &input, day2::part1);
    run("part2", &input, day2::part2);
}
