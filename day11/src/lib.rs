use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Cell {
    fn from_char(input: char) -> Cell {
        match input {
            '.' => Cell::Floor,
            'L' => Cell::EmptySeat,
            '#' => Cell::OccupiedSeat,
            _ => unreachable!(),
        }
    }

    fn floor(&self) -> bool {
        self == &Cell::Floor
    }

    fn occupied(&self) -> bool {
        self == &Cell::OccupiedSeat
    }

    fn empty(&self) -> bool {
        self == &Cell::EmptySeat
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Cell::Floor => '.',
            Cell::EmptySeat => 'L',
            Cell::OccupiedSeat => '#',
        };

        write!(f, "{}", char)
    }
}

type Matrix = Vec<Vec<Cell>>;

struct Map {
    matrix: Matrix,
    height: i32,
    width: i32,
}

impl Map {
    fn from_input(input: &Vec<&str>) -> Self {
        let map = input
            .iter()
            .map(|line| line.chars().map(|char| Cell::from_char(char)).collect())
            .collect::<Matrix>();

        let height = map.len() as i32;
        let width = map[0].len() as i32;

        Map {
            matrix: map,
            height,
            width,
        }
    }

    fn map_matrix<F>(&self, mut f: F) -> Matrix
    where
        F: FnMut(usize, usize, &Cell) -> Cell,
    {
        self.matrix
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, cell)| f(x, y, cell))
                    .collect()
            })
            .collect::<Matrix>()
    }

    fn neighbors_coords() -> [(i32, i32); 8] {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
    }

    fn occupation(&self) -> usize {
        self.matrix.iter().fold(0, |sum, row| {
            sum + row
                .iter()
                .fold(0, |sum, cell| sum + if cell.occupied() { 1 } else { 0 })
        })
    }

    fn neighbor(&self, x: usize, y: usize, mx: &i32, my: &i32) -> Option<(&Cell, usize, usize)> {
        let x = x as i32 + mx;
        let y = y as i32 + my;

        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        Some((&self.matrix[y][x], x, y))
    }

    fn occupied_seat(&self, x: usize, y: usize, mx: &i32, my: &i32) -> usize {
        let neighbor = self.neighbor(x, y, mx, my);

        if neighbor.is_some() && neighbor.unwrap().0.occupied() {
            1
        } else {
            0
        }
    }

    fn occupied_adjacent_seats(&self, x: usize, y: usize) -> usize {
        Self::neighbors_coords().iter().fold(0, |count, (mx, my)| {
            count + self.occupied_seat(x, y, mx, my)
        })
    }

    fn seat_seen(&self, x: usize, y: usize, mx: &i32, my: &i32) -> usize {
        let neighbor = self.neighbor(x, y, mx, my);

        if neighbor.is_none() {
            return 0;
        }

        let (neighbor, x, y) = neighbor.unwrap();

        if false == neighbor.floor() {
            return if neighbor.occupied() { 1 } else { 0 };
        }

        self.seat_seen(x, y, mx, my)
    }

    fn occupied_seen_seats(&self, x: usize, y: usize) -> usize {
        Self::neighbors_coords()
            .iter()
            .fold(0, |count, (mx, my)| count + self.seat_seen(x, y, mx, my))
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.matrix.iter() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn part1(input: &Vec<&str>) -> usize {
    let mut map = Map::from_input(input);
    let mut stabilized = false;

    while false == stabilized {
        stabilized = true;

        map.matrix = map.map_matrix(|x, y, cell| {
            if cell.floor() {
                return Cell::Floor;
            }

            let occupied_adjacent = map.occupied_adjacent_seats(x, y);

            if cell.empty() && occupied_adjacent == 0 {
                stabilized = false;
                return Cell::OccupiedSeat;
            }

            if cell.occupied() && occupied_adjacent >= 4 {
                stabilized = false;
                return Cell::EmptySeat;
            }

            cell.to_owned()
        });
    }

    map.occupation()
}

pub fn part2(input: &Vec<&str>) -> usize {
    let mut map = Map::from_input(input);
    let mut stabilized = false;

    while false == stabilized {
        stabilized = true;

        map.matrix = map.map_matrix(|x, y, cell| {
            let occupied_adjacent = map.occupied_seen_seats(x, y);

            if cell.empty() && occupied_adjacent == 0 {
                stabilized = false;
                return Cell::OccupiedSeat;
            }

            if cell.occupied() && occupied_adjacent >= 5 {
                stabilized = false;
                return Cell::EmptySeat;
            }

            return cell.to_owned();
        })
    }

    map.occupation()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 37)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 26)
    }
}
