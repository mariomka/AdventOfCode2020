use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "e" => Ok(Direction::East),
            "se" => Ok(Direction::Southeast),
            "sw" => Ok(Direction::Southwest),
            "w" => Ok(Direction::West),
            "nw" => Ok(Direction::Northwest),
            "ne" => Ok(Direction::Northeast),
            _ => Err(()),
        }
    }
}

fn parse(input: &Vec<&str>) -> Vec<Vec<Direction>> {
    let mut steps_list = Vec::new();

    for identifier in input.into_iter() {
        let mut steps: Vec<Direction> = Vec::new();
        let mut current_step = "".to_owned();

        for char in identifier.chars() {
            if char == 's' || char == 'n' {
                current_step.push(char);
            } else {
                current_step.push(char);
                steps.push(current_step.parse().unwrap());
                current_step = "".to_owned();
            }
        }

        steps_list.push(steps);
    }

    steps_list
}

fn calc_grid(input: &Vec<&str>) -> HashMap<(i32, i32), bool> {
    let steps_list = parse(input);
    let mut grid: HashMap<(i32, i32), bool> = HashMap::new();

    for steps in steps_list {
        let mut x: f64 = 0.0;
        let mut y = 0;

        for step in steps.iter() {
            match step {
                Direction::East => {
                    x += 1.0;
                }
                Direction::Southeast => {
                    x += 0.5;
                    y -= 1;
                }
                Direction::Southwest => {
                    x -= 0.5;
                    y -= 1;
                }
                Direction::West => {
                    x -= 1.0;
                }
                Direction::Northwest => {
                    x -= 0.5;
                    y += 1;
                }
                Direction::Northeast => {
                    x += 0.5;
                    y += 1;
                }
            }
        }

        let x_ = x.floor() as i32;

        let tile = grid.get(&(x_, y));

        if tile.is_some() {
            let tile = tile.unwrap().to_owned();
            grid.insert((x_, y), !tile);
        } else {
            grid.insert((x_, y), true);
        }
    }

    grid
}

fn black_neighbors(grid: &HashMap<(i32, i32), bool>, x: i32, y: i32) -> usize {
    let mut black_neighbors = 0;

    if y % 2 == 0 {
        if *grid.get(&(x + 1, y)).unwrap_or(&false) {
            black_neighbors += 1;
        }

        if *grid.get(&(x, y + 1)).unwrap_or(&false) {
            black_neighbors += 1;
        }

        if *grid.get(&(x - 1, y + 1)).unwrap_or(&false) {
            black_neighbors += 1;
        }

        if *grid.get(&(x - 1, y)).unwrap_or(&false) {
            black_neighbors += 1;
        }

        if *grid.get(&(x - 1, y - 1)).unwrap_or(&false) {
            black_neighbors += 1;
        }

        if *grid.get(&(x, y - 1)).unwrap_or(&false) {
            black_neighbors += 1;
        }
    } else {
        if *grid.get(&(x + 1, y)).unwrap_or(&false) {
            black_neighbors += 1;
        }

        if *grid.get(&(x + 1, y + 1)).unwrap_or(&false) {
            black_neighbors += 1;
        }

        if *grid.get(&(x, y + 1)).unwrap_or(&false) {
            black_neighbors += 1;
        }

        if *grid.get(&(x - 1, y)).unwrap_or(&false) {
            black_neighbors += 1;
        }

        if *grid.get(&(x, y - 1)).unwrap_or(&false) {
            black_neighbors += 1;
        }

        if *grid.get(&(x + 1, y - 1)).unwrap_or(&false) {
            black_neighbors += 1;
        }
    }

    black_neighbors
}

pub fn part1(input: &Vec<&str>) -> usize {
    let grid = calc_grid(input);

    let mut black_tiles_count = 0;
    for (_, tile) in grid.iter() {
        if *tile {
            black_tiles_count += 1;
        }
    }

    black_tiles_count
}

pub fn part2(input: &Vec<&str>) -> usize {
    let mut grid = calc_grid(input);

    let mut x_max = 0;
    let mut x_min = 0;
    let mut y_max = 0;
    let mut y_min = 0;

    for (&(x, y), _) in grid.iter() {
        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }
        if y < y_min {
            y_min = y;
        }
        if y > y_max {
            y_max = y;
        }
    }

    let mut x_range = x_min..=x_max;
    let mut y_range = y_min..=y_max;

    for _ in 0..100 {
        let mut new_grid: HashMap<(i32, i32), bool> = HashMap::new();

        x_range = x_range.start() - 2..=x_range.end() + 2;
        y_range = y_range.start() - 2..=y_range.end() + 2;

        for y in y_range.clone() {
            for x in x_range.clone() {
                let black_neighbors = black_neighbors(&grid, x, y);
                let mut status = *grid.get(&(x, y)).unwrap_or(&false);

                if status && (black_neighbors == 0 || black_neighbors > 2) {
                    status = false;
                } else if false == status && black_neighbors == 2 {
                    status = true;
                }

                new_grid.insert((x, y), status);
            }
        }

        grid = new_grid;
    }

    let mut black_tiles_count = 0;
    for (_, &tile) in grid.iter() {
        if tile {
            black_tiles_count += 1;
        }
    }

    black_tiles_count
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 10)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 2208)
    }
}
