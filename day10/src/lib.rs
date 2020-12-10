use std::collections::HashMap;

pub fn part1(input: &Vec<usize>) -> usize {
    let mut jolts = input.clone();
    jolts.push(jolts.iter().max().unwrap() + 3);
    jolts.sort();

    let mut jolt_differences = [0usize, 0usize, 0usize];
    let mut current_jolt = 0;

    for jolt in jolts.iter() {
        let jolt_difference = jolt - current_jolt;

        match jolt_difference {
            1 => {
                jolt_differences[0] += 1;
            }
            2 => {
                jolt_differences[1] += 1;
            }
            3 => {
                jolt_differences[2] += 1;
            }
            _ => {
                unreachable!();
            }
        }

        current_jolt += jolt_difference;
    }

    jolt_differences[0] * jolt_differences[2]
}

pub fn part2(input: &Vec<usize>) -> usize {
    let mut jolts = input.clone();

    let last_jolt = jolts.iter().max().unwrap() + 3;
    jolts.push(0usize);
    jolts.push(last_jolt);
    jolts.sort();

    let mut cache: HashMap<usize, usize> = HashMap::new();
    cache.insert(0, 1);

    for jolt in jolts.into_iter().skip(1) {
        let mut count = 0;

        if jolt >= 3 {
            count += cache.get(&(jolt - 3)).unwrap_or(&0usize);
        }

        if jolt >= 2 {
            count += cache.get(&(jolt - 2)).unwrap_or(&0usize);
        }

        count += cache.get(&(jolt - 1)).unwrap_or(&0usize);

        cache.insert(jolt, count);
    }

    *cache.get(&last_jolt).unwrap()
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
