use helpers::run;

fn main() {
    let input1 = include_str!("../input.txt");

    run("part1", || day4::part1(input1));
    run("part2", || day4::part2(input1));
}
