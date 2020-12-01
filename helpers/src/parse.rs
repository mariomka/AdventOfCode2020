use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

pub fn parse_input_vec<T: FromStr>(input: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    input
        .split_ascii_whitespace()
        .map(|line: &str| line.parse().unwrap())
        .collect()
}

pub fn parse_input_hashset<T: Eq + Hash + FromStr>(input: &str) -> HashSet<T>
where
    <T as FromStr>::Err: Debug,
{
    input
        .split_ascii_whitespace()
        .map(|line: &str| line.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_parse_input_vec() {
        let input = "
            1721
            979
            366

            299
            675
            1456
        ";

        let result: Vec<u64> = parse_input_vec(input);

        let expected: Vec<u64> = Vec::from([1721, 979, 366, 299, 675, 1456]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_input_hashset() {
        let input = "
            1721
            979
            366

            299
            675
            1456
        ";

        let result: HashSet<u64> = parse_input_hashset(input);

        let expected: HashSet<u64> = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect();
        assert_eq!(result, expected);
    }
}
