use std::collections::HashSet;

pub fn part1(input: &HashSet<u64>) -> u64 {
    for entry_a in input {
        let entry_b = 2020 - entry_a;

        if input.contains(&entry_b) {
            return entry_a * entry_b;
        }
    }

    0
}

pub fn part2(input: &HashSet<u64>) -> u64 {
    for entry_a in input {
        for entry_b in input {
            if 2020 < entry_a + entry_b {
                continue;
            }

            let entry_c = 2020 - entry_a - entry_b;

            if input.contains(&entry_c) {
                return entry_a * entry_b * entry_c;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use helpers::parse_input;

    use super::*;

    fn input() -> HashSet<u64> {
        let input = "
1721
979
366
299
675
1456
";
        parse_input(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 514579)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 241861950)
    }
}
