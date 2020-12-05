use std::collections::HashSet;

pub fn part1(input: &Vec<&str>) -> usize {
    input.iter().fold(0, |highest, line| {
        let seat_id = calc_seat_id(line);

        if seat_id > highest {
            seat_id
        } else {
            highest
        }
    })
}

pub fn part2(input: &Vec<&str>) -> usize {
    let (seat_ids, min, max) = input.iter().fold(
        (HashSet::<usize>::new(), usize::max_value(), 0),
        |(mut seat_ids, min, max), line| {
            let seat_id = calc_seat_id(line);
            seat_ids.insert(seat_id);

            (
                seat_ids,
                if min > seat_id { seat_id } else { min },
                if max < seat_id { seat_id } else { max },
            )
        },
    );

    (min..max)
        .find(|seat_id| false == seat_ids.contains(seat_id))
        .unwrap()
}

fn calc_seat_id(boarding_pass: &str) -> usize {
    boarding_pass.chars().fold(0, |num, char| {
        (num << 1) + if 'B' == char || 'R' == char { 1 } else { 0 }
    })
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
";
        input_lines(input)
    }

    #[test]
    fn test_calc_seat_id() {
        assert_eq!(calc_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(calc_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(calc_seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 820)
    }
}
