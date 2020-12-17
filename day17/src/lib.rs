use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

use fxhash::FxHashMap;
use itertools::iproduct;

#[derive(PartialEq, Debug)]
enum Cube {
    Active,
    Inactive,
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cube::Active => write!(f, "{}", "#")?,
            Cube::Inactive => write!(f, "{}", ".")?,
        };

        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
struct Bounds(i8, i8);

impl Bounds {
    fn update(&mut self, number: i8) {
        if number < self.0 {
            self.0 = number;
        }

        if number > self.1 {
            self.1 = number;
        }
    }

    fn range(self) -> RangeInclusive<i8> {
        self.0..=self.1
    }

    fn next_range(self) -> RangeInclusive<i8> {
        self.0 - 1..=self.1 + 1
    }
}

type Matrix = FxHashMap<(i8, i8, i8, i8), Cube>;

struct Pocket {
    x_bounds: Bounds,
    y_bounds: Bounds,
    z_bounds: Bounds,
    w_bounds: Bounds,
    matrix: Matrix,
}

impl Pocket {
    fn new() -> Self {
        Self {
            x_bounds: Bounds(0, 0),
            y_bounds: Bounds(0, 0),
            z_bounds: Bounds(0, 0),
            w_bounds: Bounds(0, 0),
            matrix: FxHashMap::default(),
        }
    }

    fn from_input(input: &Vec<&str>) -> Self {
        let y_len = input.len();
        let x_len = input[0].len();

        let mut pocket = Self::new();
        input.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                if '#' == char {
                    pocket.activate_cube(
                        x as i8 - (x_len - 2) as i8,
                        y as i8 - (y_len - 2) as i8,
                        0i8,
                        0i8,
                    );
                }
            })
        });

        pocket
    }

    fn is_active(&self, x: i8, y: i8, z: i8, w: i8) -> bool {
        Cube::Active == *self.get(x, y, z, w)
    }

    fn get(&self, x: i8, y: i8, z: i8, w: i8) -> &Cube {
        self.matrix.get(&(x, y, z, w)).unwrap_or(&Cube::Inactive)
    }

    fn activate_cube(&mut self, x: i8, y: i8, z: i8, w: i8) {
        self.matrix.insert((x, y, z, w), Cube::Active);
        self.x_bounds.update(x);
        self.y_bounds.update(y);
        self.z_bounds.update(z);
        self.w_bounds.update(w);
    }

    fn next_cycle(&self, with_w: bool) -> Self {
        let mut pocket = Pocket::new();

        let x_range = self.x_bounds.next_range();
        let y_range = self.y_bounds.next_range();
        let z_range = self.z_bounds.next_range();
        let w_range = if with_w {
            self.w_bounds.next_range()
        } else {
            0..=0
        };

        for (x, y, z, w) in iproduct!(x_range, y_range, z_range, w_range) {
            let cube = self.get(x, y, z, w);
            let active_neighbors = self.active_neighbors(x, y, z, w, with_w);

            match cube {
                Cube::Active => {
                    if (2..=3).contains(&active_neighbors) {
                        pocket.activate_cube(x, y, z, w);
                    }
                }
                Cube::Inactive => {
                    if 3 == active_neighbors {
                        pocket.activate_cube(x, y, z, w);
                    }
                }
            }
        }

        pocket
    }

    fn active_neighbors(&self, x: i8, y: i8, z: i8, w: i8, with_w: bool) -> i8 {
        let x_range = x - 1..=x + 1;
        let y_range = y - 1..=y + 1;
        let z_range = z - 1..=z + 1;
        let w_range = if with_w { w - 1..=w + 1 } else { 0..=0 };

        iproduct!(x_range, y_range, z_range, w_range).fold(0, |sum, (x2, y2, z2, w2)| {
            sum + if false == (x == x2 && y == y2 && z == z2 && w == w2)
                && self.is_active(x2, y2, z2, w2)
            {
                1
            } else {
                0
            }
        })
    }
}

impl Display for Pocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for w in self.w_bounds.range() {
            for z in self.z_bounds.range() {
                writeln!(f, "z={} w={}", z, w)?;
                for y in self.y_bounds.range() {
                    for x in self.x_bounds.range() {
                        let cube = self.get(x, y, z, w);
                        write!(f, "{}", cube)?;
                    }

                    writeln!(f)?;
                }
            }
        }

        Ok(())
    }
}

pub fn part1(input: &Vec<&str>) -> usize {
    let pocket = (0..6).fold(Pocket::from_input(input), |pocket, _| {
        pocket.next_cycle(false)
    });

    pocket.matrix.len()
}

pub fn part2(input: &Vec<&str>) -> usize {
    let pocket = (0..6).fold(Pocket::from_input(input), |pocket, _| {
        pocket.next_cycle(true)
    });

    pocket.matrix.len()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
.#.
..#
###";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 112)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 848)
    }
}
