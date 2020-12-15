use helpers::debug;
use std::collections::HashMap;

fn calc_nth(input: &Vec<usize>, nth: usize) -> usize {
    let mut said: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut last_number = 0;

    for index in 0..nth {
        last_number = if index < input.len() {
            let number = input[index];
            said.insert(number, (index, index));

            number
        } else {
            let (turn_a, turn_b) = said.get(&last_number).unwrap();
            let number = turn_a - turn_b;

            if said.contains_key(&number) {
                let previous = said.get(&number).unwrap();

                said.insert(number, (index, previous.0));
            } else {
                said.insert(number, (index, index));
            }

            number
        }
    }

    last_number
}

pub fn part1(input: &Vec<usize>) -> usize {
    calc_nth(input, 2020)
}

pub fn part2(input: &Vec<usize>) -> usize {
    calc_nth(input, 30_000_000)
}

#[cfg(test)]
mod tests {
    use helpers::parse_split_input;

    use super::*;

    #[test]
    fn test_part1_a() {
        assert_eq!(part1(&parse_split_input("0,3,6", ",")), 436)
    }

    #[test]
    fn test_part1_b() {
        assert_eq!(part1(&parse_split_input("2,1,3", ",")), 10)
    }

    #[test]
    fn test_part1_c() {
        assert_eq!(part1(&parse_split_input("1,2,3", ",")), 27)
    }

    #[test]
    fn test_part1_d() {
        assert_eq!(part1(&parse_split_input("2,3,1", ",")), 78)
    }

    #[test]
    fn test_part1_e() {
        assert_eq!(part1(&parse_split_input("3,2,1", ",")), 438)
    }

    #[test]
    fn test_part1_f() {
        assert_eq!(part1(&parse_split_input("3,1,2", ",")), 1836)
    }

    #[test]
    fn test_part2_a() {
        assert_eq!(part2(&parse_split_input("0,3,6", ",")), 175594)
    }

    #[test]
    fn test_part2_b() {
        assert_eq!(part2(&parse_split_input("1,3,2", ",")), 2578)
    }

    #[test]
    fn test_part2_c() {
        assert_eq!(part2(&parse_split_input("2,1,3", ",")), 3544142)
    }

    #[test]
    fn test_part2_d() {
        assert_eq!(part2(&parse_split_input("1,2,3", ",")), 261214)
    }

    #[test]
    fn test_part2_e() {
        assert_eq!(part2(&parse_split_input("2,3,1", ",")), 6895259)
    }

    #[test]
    fn test_part2_f() {
        assert_eq!(part2(&parse_split_input("3,2,1", ",")), 18)
    }

    #[test]
    fn test_part2_g() {
        assert_eq!(part2(&parse_split_input("3,1,2", ",")), 362)
    }
}
