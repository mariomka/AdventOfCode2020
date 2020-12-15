use std::fmt::Debug;
use std::iter::FromIterator;
use std::str::FromStr;

pub fn input_lines<'a, R>(input: &'a str) -> R
where
    R: FromIterator<&'a str>,
{
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| false == line.is_empty())
        .collect()
}

pub fn split_input<'a, R>(input: &'a str, pattern: &str) -> R
where
    R: FromIterator<&'a str>,
{
    input
        .split(pattern)
        .map(|line| line.trim())
        .filter(|line| false == line.is_empty())
        .collect()
}

pub fn parse_split_input<T: FromStr, R>(input: &str, pattern: &str) -> R
where
    T::Err: Debug,
    R: FromIterator<T>,
{
    input
        .split(pattern)
        .map(|line| line.trim())
        .filter(|line| false == line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn parse_input<T: FromStr, R>(input: &str) -> R
where
    T::Err: Debug,
    R: FromIterator<T>,
{
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| false == line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_input_lines_to_vec() {
        let input = "
            1-3 a: abcde
            1-3 b: cdefg

            2-9 c: ccccccccc
        ";

        let result: Vec<&str> = input_lines(input);

        let expected: Vec<&str> = Vec::from(["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_input_lines_to_hashset() {
        let input = "
            1-3 a: abcde
            1-3 b: cdefg

            2-9 c: ccccccccc
        ";

        let result: HashSet<&str> = input_lines(input);

        let expected: HashSet<&str> = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
            .iter()
            .cloned()
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_lines_to_vec() {
        let input = "
            1-3 a: abcde

            1-3 b: cdefg

            2-9 c: ccccccccc
        ";

        let result: Vec<&str> = split_input(input, "\n\n");

        let expected: Vec<&str> = Vec::from(["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_lines_to_hashset() {
        let input = "
            1-3 a: abcde

            1-3 b: cdefg

            2-9 c: ccccccccc
        ";

        let result: HashSet<&str> = split_input(input, "\n\n");

        let expected: HashSet<&str> = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
            .iter()
            .cloned()
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_input_to_vec() {
        let input = "
            1721
            979
            366

            299
            675
            1456
        ";

        let result: Vec<u64> = parse_input(input);

        let expected: Vec<u64> = Vec::from([1721, 979, 366, 299, 675, 1456]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_input_to_hashset() {
        let input = "
            1721
            979
            366

            299
            675
            1456
        ";

        let result: HashSet<u64> = parse_input(input);

        let expected: HashSet<u64> = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect();
        assert_eq!(result, expected);
    }
}
