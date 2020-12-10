pub fn prepare_input(input: &Vec<usize>) -> Vec<usize> {
    let mut prepared = input.clone();
    prepared.push(0usize);
    prepared.sort();
    prepared.push(prepared.last().unwrap() + 3);

    prepared
}

pub fn part1(input: &Vec<usize>) -> usize {
    let input = prepare_input(input);

    let mut jolt_differences = (0usize, 0usize);

    for jolt_window in input.windows(2) {
        match jolt_window[1] - jolt_window[0] {
            1 => {
                jolt_differences.0 += 1;
            }
            3 => {
                jolt_differences.1 += 1;
            }
            _ => {
                unreachable!();
            }
        }
    }

    jolt_differences.0 * jolt_differences.1
}

pub fn part2(input: &Vec<usize>) -> usize {
    let input = prepare_input(input);

    let mut counts = vec![0usize; input.last().unwrap() + 1];
    counts[0] = 1;

    for jolt in input.into_iter().skip(1) {
        let mut count = 0;

        if jolt >= 3 {
            count += counts[jolt - 3];
        }

        if jolt >= 2 {
            count += counts[jolt - 2];
        }

        count += counts[jolt - 1];

        counts[jolt] = count;
    }

    *counts.last().unwrap()
}

#[cfg(test)]
mod tests {
    use helpers::parse_input;

    use super::*;

    fn input_a() -> Vec<usize> {
        let input = "
16
10
15
5
1
11
7
19
6
12
4";
        parse_input(input)
    }

    fn input_b() -> Vec<usize> {
        let input = "
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        parse_input(input)
    }

    #[test]
    fn test_part1_a() {
        assert_eq!(part1(&input_a()), 7 * 5)
    }

    #[test]
    fn test_part1_b() {
        assert_eq!(part1(&input_b()), 22 * 10)
    }

    #[test]
    fn test_part2_a() {
        assert_eq!(part2(&input_a()), 8)
    }

    #[test]
    fn test_part2_b() {
        assert_eq!(part2(&input_b()), 19208)
    }
}
