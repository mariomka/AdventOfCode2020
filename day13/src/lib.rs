pub fn parse_buses(line: &str) -> Vec<usize> {
    line.split(",")
        .map(|bus_id| {
            if bus_id == "x" {
                0
            } else {
                bus_id.parse::<usize>().unwrap()
            }
        })
        .collect::<Vec<usize>>()
}

pub fn part1(input: &Vec<&str>) -> usize {
    let depart_time = input[0].parse::<usize>().unwrap();
    let buses = parse_buses(input[1]);

    let mut shorter_waiting_time = usize::max_value();
    let mut earliest_bust = 0;

    for bus in buses {
        if 0 == bus {
            continue;
        }

        let waiting_time = (bus * (depart_time as f64 / bus as f64).ceil() as usize) - depart_time;

        if waiting_time < shorter_waiting_time {
            shorter_waiting_time = waiting_time;
            earliest_bust = bus;
        }
    }

    earliest_bust * shorter_waiting_time
}

pub fn part2(input: &Vec<&str>) -> usize {
    let buses = parse_buses(input[1]);

    let mut timestamp = 0;
    let mut bus_index = 1;
    let mut increment = buses[0];

    loop {
        if buses[bus_index] == 0 {
            bus_index += 1;
            continue;
        }

        if (timestamp + bus_index) % buses[bus_index] == 0 {
            increment *= buses[bus_index];
            bus_index += 1;
        }

        if bus_index >= buses.len() {
            break;
        }

        timestamp += increment;
    }

    timestamp
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
939
7,13,x,x,59,x,31,19
";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 295)
    }

    #[test]
    fn test_part2_a() {
        assert_eq!(part2(&input()), 1068781)
    }

    #[test]
    fn test_part2_b() {
        let input = input_lines(
            "0
17,x,13,19",
        );
        assert_eq!(part2(&input), 3417)
    }

    #[test]
    fn test_part2_c() {
        let input = input_lines(
            "0
67,7,59,61",
        );
        assert_eq!(part2(&input), 754018)
    }

    #[test]
    fn test_part2_d() {
        let input = input_lines(
            "0
67,x,7,59,61",
        );
        assert_eq!(part2(&input), 779210)
    }

    #[test]
    fn test_part2_e() {
        let input = input_lines(
            "0
67,7,x,59,61",
        );
        assert_eq!(part2(&input), 1261476)
    }

    #[test]
    fn test_part2_f() {
        let input = input_lines(
            "0
1789,37,47,1889",
        );
        assert_eq!(part2(&input), 1202161486)
    }
}
