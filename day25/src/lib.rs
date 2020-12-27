fn transform(subject: usize, loop_size: usize) -> usize {
    let mut value = 1;

    for _ in 0..loop_size {
        value = (value * subject) % 20201227;
    }

    value
}

pub fn part1(input: &Vec<usize>) -> usize {
    let card_public_key = input[0];
    let door_public_key = input[1];
    let subject = 7;

    let mut card_loop_size = 0;
    let mut value = 1;

    for loop_number in 1..usize::max_value() {
        value = (value * subject) % 20201227;

        if value == card_public_key {
            card_loop_size = loop_number;
            break;
        }
    }

    transform(door_public_key, card_loop_size)
}

#[cfg(test)]
mod tests {
    use helpers::parse_input;

    use super::*;

    fn input<'a>() -> Vec<usize> {
        let input = "
5764801
17807724
";
        parse_input(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 14897079)
    }
}
