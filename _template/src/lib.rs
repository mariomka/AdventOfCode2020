pub fn part1(input: &Vec<&str>) -> usize {
    0
}

pub fn part2(input: &Vec<&str>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 0)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 0)
    }
}
