use itertools::Itertools;

pub fn part1(input: &Vec<u64>, preamble_length: usize) -> u64 {
    'outer: for (index, number) in (&input[preamble_length..]).iter().enumerate() {
        let preamble = &input[index..index + preamble_length];

        for pair in preamble.iter().combinations(2) {
            if *number == (*pair.get(0).unwrap()) + *(pair.get(1).unwrap()) {
                continue 'outer;
            }
        }

        return *number;
    }

    0
}

pub fn part2(input: &Vec<u64>, preamble_length: usize) -> u64 {
    let current_number = part1(input, preamble_length);

    for (index, number) in input.iter().enumerate() {
        if current_number == *number {
            break;
        }

        let mut set = Vec::new();
        let mut sum = 0u64;
        set.push(*number);

        for next_number in input.iter().skip(index + 1) {
            set.push(*next_number);
            sum += next_number;

            if sum == current_number {
                let min = *set.iter().min().unwrap();
                let max = *set.iter().max().unwrap();

                return min + max;
            }

            if sum > current_number {
                break;
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use helpers::parse_input;

    use super::*;

    fn input<'a>() -> Vec<u64> {
        let input = "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        parse_input(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input(), 5), 127)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input(), 5), 62)
    }
}
