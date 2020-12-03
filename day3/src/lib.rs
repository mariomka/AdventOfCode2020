fn count_trees(input: &Vec<&str>, right: usize, bottom: usize) -> usize {
    let mut position = (1usize, 1usize);
    let mut trees = 0;

    let width = input.get(0).unwrap().len();

    loop {
        position = (position.0 + right, position.1 + bottom);

        if position.1 > input.len() {
            break;
        }

        if position.0 > width {
            position.0 = position.0 % width;
        }

        let line = input.get(position.1 - 1).unwrap();
        let square = line.chars().nth(position.0 - 1).unwrap();

        if '#' == square {
            trees += 1;
        }
    }

    trees
}

pub fn part1(input: &Vec<&str>) -> usize {
    count_trees(input, 3, 1)
}

pub fn part2(input: &Vec<&str>) -> usize {
    count_trees(input, 1, 1)
        * count_trees(input, 3, 1)
        * count_trees(input, 5, 1)
        * count_trees(input, 7, 1)
        * count_trees(input, 1, 2)
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 7)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 336)
    }
}
