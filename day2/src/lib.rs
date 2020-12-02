#[macro_use]
extern crate lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Password<'a> {
    num1: usize,
    num2: usize,
    char: char,
    password: &'a str,
}

impl Password<'_> {
    fn from_line(line: &str) -> Password {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"^(?P<num1>\d{1,2})-(?P<num2>\d{1,2}) (?P<char>[a-z]): (?P<password>[a-z]+)$"
            )
            .unwrap();
        }

        let captures = REGEX.captures(line).unwrap();

        Password {
            num1: captures.name("num1").unwrap().as_str().parse().unwrap(),
            num2: captures.name("num2").unwrap().as_str().parse().unwrap(),
            char: captures.name("char").unwrap().as_str().parse().unwrap(),
            password: captures.name("password").unwrap().as_str(),
        }
    }

    fn is_valid_first_policy(&self) -> bool {
        let char_count = self.password.matches(self.char).count();

        char_count >= self.num1 && char_count <= self.num2
    }

    fn is_valid_second_policy(&self) -> bool {
        let first_char = self.password.chars().nth(self.num1 - 1).unwrap();
        let second_char = self.password.chars().nth(self.num2 - 1).unwrap();

        (self.char != first_char) != (self.char != second_char)
    }
}

pub fn part1(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| Password::from_line(line))
        .filter(|password| password.is_valid_first_policy())
        .count()
}

pub fn part2(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| Password::from_line(line))
        .filter(|password| password.is_valid_second_policy())
        .count()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 2)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1)
    }
}
