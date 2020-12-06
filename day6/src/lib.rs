fn line_to_binary(line: &str) -> u32 {
    line.chars()
        .fold(0, |binary, char| binary | 1 << ((char as u8) - 97)) // 97 is the ascii code for 'a'
}

fn affirmative_answers(input: &str) -> usize {
    input
        .lines()
        .map(line_to_binary)
        .fold(0, |prev, binary| prev | binary)
        .count_ones() as usize
}

fn affirmative_group_answers(input: &str) -> usize {
    input
        .lines()
        .map(line_to_binary)
        .fold(u32::max_value(), |prev, binary| prev & binary)
        .count_ones() as usize
}

pub fn part1(input: &Vec<&str>) -> usize {
    input.iter().map(|group| affirmative_answers(group)).sum()
}

pub fn part2(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|group| affirmative_group_answers(group))
        .sum()
}

#[cfg(test)]
mod tests {
    use helpers::split_input;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
abc

a
b
c

ab
ac

a
a
a
a

b";
        split_input(input, "\n\n")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 11)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 6)
    }
}
