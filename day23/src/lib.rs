fn step_part1(cups: &mut Vec<usize>, current_cup_index: usize) -> usize {
    let mut removed_cups = Vec::new();
    let current_cup_number = cups[current_cup_index];

    for i in 1..=3 {
        let index = (current_cup_index + i) % cups.len();
        removed_cups.push(cups[index]);
    }

    cups.retain(|cup| false == removed_cups.contains(cup));

    let mut destination_number = current_cup_number;
    let destination_number_index;

    loop {
        destination_number -= 1;

        match cups.iter().position(|cup| *cup == destination_number) {
            Some(index) => {
                destination_number_index = index;
                break;
            }
            None => {
                if *cups.iter().min().unwrap() > destination_number {
                    let max = *cups.iter().max().unwrap();
                    destination_number_index = cups.iter().position(|cup| *cup == max).unwrap();
                    break;
                }
            }
        }
    }

    cups.splice(
        (destination_number_index + 1)..(destination_number_index + 1),
        removed_cups.iter().cloned(),
    );

    let current_cup_index = cups
        .iter()
        .position(|cup| *cup == current_cup_number)
        .unwrap();

    (current_cup_index + 1) % cups.len()
}

pub fn part1(input: &Vec<usize>) -> String {
    let mut cups = input.clone();

    let mut current_cup_index = 0;

    for _ in 0..100 {
        current_cup_index = step_part1(&mut cups, current_cup_index);
    }

    let mut solution = "".to_owned();

    let mut one_found = false;
    for cup in cups.iter() {
        if *cup == 1 {
            one_found = true;
        } else if one_found {
            solution.push_str(&cup.to_string());
        }
    }

    for cup in cups.iter() {
        if *cup == 1 {
            break;
        }

        solution.push_str(&cup.to_string());
    }

    solution
}

fn step_part2(min: usize, max: usize, cup_index: &mut Vec<usize>, current_cup: usize) -> usize {
    let next_1 = cup_index[current_cup];
    let next_2 = cup_index[next_1];
    let next_3 = cup_index[next_2];

    let mut destination = current_cup - 1;

    while 0 == destination
        || next_1 == destination
        || next_2 == destination
        || next_3 == destination
    {
        if destination == 0 {
            destination = max;
            continue;
        }

        destination -= 1;

        if min > destination {
            destination = max;
        }
    }

    cup_index[current_cup] = cup_index[next_3];

    let old_destination = cup_index[destination];
    cup_index[destination] = next_1;
    cup_index[next_3] = old_destination;

    cup_index[current_cup]
}

pub fn part2(input: &Vec<usize>) -> usize {
    let mut cups = Vec::new();
    cups.append(&mut input.clone());

    let max = *cups.iter().max().unwrap();
    for number in max + 1..=1_000_000 {
        cups.push(number);
    }

    let mut cup_index = vec![0; 1_000_001];

    let max = *cups.iter().max().unwrap();
    let min = *cups.iter().min().unwrap();

    let first = *cups.first().unwrap();
    let mut prev = first;
    for cup in cups.iter().skip(1) {
        cup_index[prev] = *cup;
        prev = *cup;
    }
    cup_index[prev] = first;

    let mut current_cup = first;

    for _ in 0..10_000_000 {
        current_cup = step_part2(min, max, &mut cup_index, current_cup);
    }

    cup_index[1] * cup_index[cup_index[1]]
}

#[cfg(test)]
mod tests {
    use helpers::parse_split_input;

    use super::*;

    fn input() -> Vec<usize> {
        let input = "389125467";
        parse_split_input(input, "")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), "67384529")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 149245887792)
    }
}
