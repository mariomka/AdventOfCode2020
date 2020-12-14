#[macro_use]
extern crate lazy_static;
use regex::Regex;

use std::collections::HashMap;

fn parse_line(line: &str) -> (&str, usize, &str) {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r"^(?P<instruction>(mask|mem\[(?P<address>\d+)\])) = (?P<value>.*)$")
                .unwrap();
    }

    let captures = REGEX.captures(line).unwrap();

    let instruction = captures.name("instruction").unwrap().as_str();
    let address = captures.name("address");
    let address: usize = if address.is_some() {
        address.unwrap().as_str().parse().unwrap()
    } else {
        0
    };
    let value = captures.name("value").unwrap().as_str();

    (instruction, address, value)
}

fn apply_mask(mask: &str, value: usize) -> usize {
    let mut mask_0 = usize::max_value();
    let mut mask_1 = 0usize;

    for (index, char) in mask.chars().rev().enumerate() {
        match char {
            '0' => mask_0 ^= 1 << index,
            '1' => mask_1 ^= 1 << index,
            _ => {}
        }
    }

    let value = value & mask_0 | mask_1;

    value
}

fn combine(addresses: Vec<usize>, floating_index: usize, floatings: &Vec<usize>) -> Vec<usize> {
    let mut new_addresses = Vec::new();
    let floating = floatings[floating_index];

    for address in addresses.iter() {
        let address = address & (usize::max_value() ^ (1 << floating)); // apply 0
        new_addresses.push(address);

        let address = address | (0 ^ (1 << floating)); // apply 1
        new_addresses.push(address);
    }

    if floating_index + 1 >= floatings.len() {
        return new_addresses;
    }

    combine(new_addresses, floating_index + 1, floatings)
}

fn apply_mask_v2(mask: &str, address: usize) -> Vec<usize> {
    let mut mask_1 = 0usize;
    let mut floatings = Vec::new();

    for (index, char) in mask.chars().rev().enumerate() {
        match char {
            '1' => mask_1 ^= 1 << index,
            'X' => floatings.push(index),
            _ => {}
        }
    }
    let address = address | mask_1;

    combine(vec![address], 0, &floatings)
}

pub fn part1(input: &Vec<&str>) -> usize {
    let mut current_mask = "";
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for line in input.iter() {
        let (instruction, address, value) = parse_line(line);

        if "mask" == instruction {
            current_mask = value;
        } else {
            memory.insert(
                address,
                apply_mask(current_mask.clone(), value.parse().unwrap()),
            );
        }
    }

    memory.iter().fold(0, |sum, (_, value)| sum + value)
}

pub fn part2(input: &Vec<&str>) -> usize {
    let mut current_mask = "";
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for line in input.iter() {
        let (instruction, address, value) = parse_line(line);

        if "mask" == instruction {
            current_mask = value;
        } else {
            for address in apply_mask_v2(current_mask, address) {
                memory.insert(address, value.parse().unwrap());
            }
        }
    }

    memory.iter().fold(0, |sum, (_, value)| sum + value)
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    #[test]
    fn test_part1() {
        let input = "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        assert_eq!(part1(&input_lines(input)), 165)
    }

    #[test]
    fn test_part2() {
        let input = "
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        assert_eq!(part2(&input_lines(input)), 208)
    }
}
