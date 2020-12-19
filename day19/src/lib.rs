use std::collections::{HashMap, VecDeque};

use lazy_static::lazy_static;
use regex::{Match, Regex};

#[derive(Debug)]
struct Rule {
    char: Option<char>,
    left_numbers: Option<Vec<usize>>,
    right_numbers: Option<Vec<usize>>,
}

fn is_valid(rules: &HashMap<usize, Rule>, message: &str, next_rules: VecDeque<&Rule>) -> bool {
    if message.len() == 0 {
        return next_rules.len() == 0;
    }

    let mut next_rules = next_rules.clone();

    let rule = match next_rules.pop_front() {
        None => return false,
        Some(rule) => rule,
    };

    // test char
    match rule.char {
        Some(rule_char) => {
            let message_char = message.chars().next().unwrap();

            return if rule_char == message_char {
                is_valid(rules, &message[1..], next_rules)
            } else {
                false
            };
        }
        None => {}
    }

    // test left side
    let left_numbers = rule.left_numbers.as_ref().unwrap();
    let mut left_next_rules = VecDeque::new();

    left_numbers
        .iter()
        .for_each(|number| left_next_rules.push_back(rules.get(number).unwrap()));

    left_next_rules.append(&mut next_rules.clone());

    if is_valid(rules, message, left_next_rules) {
        return true;
    }

    // test right side
    if rule.right_numbers.is_some() {
        let right_numbers = rule.right_numbers.as_ref().unwrap();
        let mut right_next_rules = VecDeque::new();

        right_numbers
            .iter()
            .for_each(|number| right_next_rules.push_back(rules.get(number).unwrap()));

        right_next_rules.append(&mut next_rules.clone());

        if is_valid(rules, message, right_next_rules) {
            return true;
        }
    }

    false
}

fn parse_char(char: Option<Match>) -> Option<char> {
    if char.is_some() {
        Some(char.unwrap().as_str()[1..2].parse().unwrap())
    } else {
        None
    }
}

fn parse_numbers(numbers: Option<Match>) -> Option<Vec<usize>> {
    if numbers.is_some() {
        Some(
            numbers
                .unwrap()
                .as_str()
                .split(" ")
                .map(|number| number.parse::<usize>().unwrap())
                .collect(),
        )
    } else {
        None
    }
}

pub fn part1(input: &Vec<&str>) -> usize {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r#"^(?P<index>\d+): (?:(?P<char>"[a-b]")|(:?(?P<left_numbers>(\d+ ?)+)(:? \| (?P<right_numbers>(\d+ ?)+))?))$"#)
                .unwrap();
    }

    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    input.iter().for_each(|line| match REGEX.captures(line) {
        Some(captures) => {
            let index: usize = captures.name("index").unwrap().as_str().parse().unwrap();
            let char = parse_char(captures.name("char"));
            let left_numbers = parse_numbers(captures.name("left_numbers"));
            let right_numbers = parse_numbers(captures.name("right_numbers"));

            rules.insert(
                index,
                Rule {
                    char,
                    left_numbers,
                    right_numbers,
                },
            );
        }
        None => messages.push(line),
    });

    messages
        .iter()
        .filter(|message| is_valid(&rules, message, VecDeque::from(vec![&rules[&0]])))
        .count()
}

pub fn part2(input: &Vec<&str>) -> usize {
    let mut input = input.clone();
    input.push("8: 42 | 42 8");
    input.push("11: 42 31 | 42 11 31");

    part1(&input)
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    #[test]
    fn test_part1() {
        let input = "
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

        assert_eq!(part1(&input_lines(input)), 2)
    }

    #[test]
    fn test_part2() {
        let input = "
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

        assert_eq!(part2(&input_lines(input)), 12)
    }
}
