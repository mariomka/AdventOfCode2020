use std::fmt::Debug;
use std::iter::FromIterator;
use std::str::FromStr;

pub fn parse_input<T: FromStr, R>(input: &str) -> R
where
    <T as FromStr>::Err: Debug,
    R: FromIterator<T>,
{
    input
        .split_ascii_whitespace()
        .map(|line: &str| line.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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
