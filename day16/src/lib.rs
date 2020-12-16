#[macro_use]
extern crate lazy_static;

use regex::Regex;

enum Reading {
    Rules,
    MyTicket,
    NearbyTickets,
}

type Ticket = Vec<usize>;
type Range = (usize, usize);

#[derive(PartialEq)]
struct Rule(Range, Range);

#[derive(PartialEq)]
struct RuleWithIndex(usize, Rule);

fn parse_input(input: &Vec<&str>) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    lazy_static! {
        static ref RULE_REGEX: Regex =
            Regex::new(r"^.*: (?P<fromA>\d+)-(?P<toA>\d+) or (?P<fromB>\d+)-(?P<toB>\d+)$")
                .unwrap();
    }

    let mut reading = Reading::Rules;
    let mut rules = Vec::new();
    let mut my_ticket = Vec::new();
    let mut nearby_tickets = Vec::new();

    for line in input {
        if line == &"your ticket:" {
            reading = Reading::MyTicket;
            continue;
        }

        if line == &"nearby tickets:" {
            reading = Reading::NearbyTickets;
            continue;
        }

        match reading {
            Reading::Rules => {
                let captures = RULE_REGEX.captures(line).unwrap();
                rules.push(Rule(
                    (
                        captures
                            .name("fromA")
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap(),
                        captures
                            .name("toA")
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap(),
                    ),
                    (
                        captures
                            .name("fromB")
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap(),
                        captures
                            .name("toB")
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap(),
                    ),
                ));
            }
            Reading::MyTicket => {
                my_ticket = line
                    .split(",")
                    .map(|part| part.parse::<usize>().unwrap())
                    .collect();
            }
            Reading::NearbyTickets => {
                nearby_tickets.push(
                    line.split(",")
                        .map(|part| part.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>(),
                );
            }
        }
    }
    (rules, my_ticket, nearby_tickets)
}

fn test_rule(number: &usize, rule: &Rule) -> bool {
    let range_a = rule.0;
    let range_b = rule.1;

    (*number >= range_a.0 && *number <= range_a.1) || (*number >= range_b.0 && *number <= range_b.1)
}

fn solve<'a>(
    deep: usize,
    rule_list: &'a Vec<RuleWithIndex>,
    tickets: &Vec<Ticket>,
    current_rules: Vec<&'a RuleWithIndex>,
) -> Option<Vec<&'a RuleWithIndex>> {
    for rule in rule_list.iter() {
        if current_rules.contains(&rule) {
            continue;
        }

        if tickets.iter().all(|ticket| {
            let number = ticket[deep];

            test_rule(&number, &rule.1)
        }) {
            let mut current_rules = current_rules.clone();
            current_rules.push(rule);

            if current_rules.len() == rule_list.len() {
                return Some(current_rules);
            }

            let result = solve(deep + 1, rule_list, tickets, current_rules);

            if result.is_some() {
                return result;
            }
        }
    }

    return None;
}

pub fn part1(input: &Vec<&str>) -> usize {
    let (rules, _, nearby_tickets) = parse_input(input);

    let mut ticket_scanning_error_rate = 0;

    for nearby_ticket in nearby_tickets {
        'numbers: for number in nearby_ticket {
            for rule in rules.iter() {
                if test_rule(&number, rule) {
                    continue 'numbers;
                }
            }

            ticket_scanning_error_rate += number;
        }
    }

    ticket_scanning_error_rate
}

pub fn part2(input: &Vec<&str>) -> usize {
    let (rules, my_ticket, nearby_tickets) = parse_input(input);
    let rules_with_index = rules
        .iter()
        .enumerate()
        .map(|(index, rule)| RuleWithIndex(index, Rule(rule.0, rule.1)))
        .collect::<Vec<RuleWithIndex>>();

    let mut valid_tickets = Vec::new();
    valid_tickets.push(my_ticket.clone());

    'tickets: for nearby_ticket in nearby_tickets.iter() {
        'numbers: for number in nearby_ticket {
            for rule_with_index in rules_with_index.iter() {
                if test_rule(number, &rule_with_index.1) {
                    continue 'numbers;
                }
            }

            continue 'tickets;
        }

        valid_tickets.push(nearby_ticket.clone());
    }

    let result = solve(0, &rules_with_index, &valid_tickets, Vec::new()).unwrap();

    result
        .iter()
        .enumerate()
        .fold(1, |solution, (index, rule)| {
            let name = input[rule.0].split(":").collect::<Vec<&str>>()[0];

            if name.starts_with("departure") {
                return solution * my_ticket[index];
            }

            solution
        })
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    #[test]
    fn test_part1() {
        let input = "
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        assert_eq!(part1(&input_lines(input)), 71)
    }
}
