#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use regex::{Captures, Regex};

pub mod graphs;

#[derive(Debug)]
struct Bag<'a> {
    children: Vec<Child<'a>>,
}

#[derive(Debug)]
struct Child<'a> {
    color: &'a str,
    quantity: u8,
}

fn parse_bags<'a>(input: &'a Vec<&str>) -> HashMap<&'a str, Bag<'a>> {
    lazy_static! {
        static ref REGEX_BAG: Regex =
            Regex::new(r"^([a-z]+ [a-z]+) bags contain (no other bags.)?").unwrap();
        static ref REGEX_CHILDREN: Regex =
            Regex::new(r"([0-9]+) ([a-z]+ [a-z]+) bags?[,.]").unwrap();
    }

    let mut bags: HashMap<&str, Bag> = HashMap::new();

    for line in input.iter() {
        let captures: Captures = REGEX_BAG.captures(line).unwrap();

        let color = captures.get(1).unwrap().as_str();
        let mut children: Vec<Child> = Vec::new();

        // Bag has children
        if captures.get(2).is_none() {
            for capture in REGEX_CHILDREN.captures_iter(line) {
                let quantity = capture.get(1).unwrap().as_str().parse::<u8>().unwrap();
                let color = capture.get(2).unwrap().as_str();

                children.push(Child { color, quantity });
            }
        }

        bags.insert(color, Bag { children });
    }

    bags
}

pub fn part1(input: &Vec<&str>) -> usize {
    let bags = parse_bags(input);
    let mut colors = vec!["shiny gold"];

    loop {
        let mut new_colors = Vec::new();

        for (bag_color, bag) in bags.iter() {
            if false == colors.contains(bag_color)
                && bag
                    .children
                    .iter()
                    .any(|child| colors.contains(&child.color))
            {
                new_colors.push(*bag_color);
            }
        }

        if 0 == new_colors.len() {
            break;
        }

        colors.append(&mut new_colors);
    }

    colors.len() - 1
}

pub fn part2(input: &Vec<&str>) -> u32 {
    let bags = parse_bags(input);

    let mut count = 0u32;
    let mut colors = vec![(1u32, "shiny gold")];

    loop {
        let mut new_colors = Vec::new();

        for (multiplier, color) in colors.iter() {
            let bag = bags.get(color).unwrap();

            for child in bag.children.iter() {
                let times = multiplier * (child.quantity as u32);
                new_colors.push((times, child.color));
                count += times;
            }
        }

        if 0 == new_colors.len() {
            break;
        }

        colors = new_colors;
    }

    count
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    #[test]
    fn test_part1() {
        let input = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

        assert_eq!(part1(&input_lines(input)), 4);
    }

    #[test]
    fn test_part2() {
        let input = "
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

        assert_eq!(part2(&input_lines(input)), 126);
    }
}
