use std::fmt::{Debug, Display, Formatter};

use helpers::debug;
use itertools::Itertools;

type Matrix = Vec<Vec<bool>>;

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    matrix: Matrix,
    rotation: usize,
    flip_x: bool,
    flip_y: bool,
}

impl Tile {
    fn from_chunk(chunk: &[&str]) -> Self {
        let id: usize = (&chunk[0][5..=8]).parse().unwrap();
        let mut matrix = vec![vec![false; 10]; 10];

        for (y, line) in chunk[1..].iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    matrix[y][x] = true;
                }
            }
        }

        Self {
            id,
            matrix,
            rotation: 0,
            flip_x: false,
            flip_y: false,
        }
    }

    fn orientations(&self) -> TileOrientations {
        TileOrientations::new((*self).clone())
    }

    fn rotate(&mut self) {
        let mut matrix = vec![vec![false; 10]; 10];

        for (y, row) in self.matrix.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                matrix[x][9 - y] = *cell;
            }
        }

        self.rotation = (self.rotation + 90) % 360;
        self.matrix = matrix;
    }

    fn flip_x(&mut self) {
        let mut matrix = vec![vec![false; 10]; 10];

        for (y, row) in self.matrix.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                matrix[y][9 - x] = *cell;
            }
        }

        self.flip_x = !self.flip_x;
        self.matrix = matrix;
    }

    fn flip_y(&mut self) {
        let mut matrix = vec![vec![false; 10]; 10];

        for (y, row) in self.matrix.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                matrix[9 - y][x] = *cell;
            }
        }

        self.flip_y = !self.flip_y;
        self.matrix = matrix;
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Tile ({} x: {} y: {}) {}:",
            self.rotation, self.flip_x, self.flip_y, self.id
        )?;
        for row in self.matrix.iter() {
            for cell in row {
                write!(f, "{}", if true == *cell { "#" } else { "." })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

struct TileOrientations {
    index: usize,
    tile: Tile,
}

impl TileOrientations {
    fn new(tile: Tile) -> Self {
        Self { index: 0, tile }
    }
}

impl Iterator for TileOrientations {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index % 4 {
            0 => {
                if self.index > 0 {
                    self.tile.flip_y();
                    self.tile.rotate();
                }
            }
            1 => self.tile.flip_x(),
            2 => self.tile.flip_y(),
            3 => self.tile.flip_x(),
            _ => unreachable!(),
        }

        self.index += 1;

        // 2 rotations are enough to
        // generate all variations
        if self.index <= 4 * 2 {
            Some(self.tile.clone())
        } else {
            None
        }
    }
}

fn parse_input(input: &Vec<&str>) -> Vec<Tile> {
    let mut tiles = Vec::new();

    for chunk in input.chunks(11) {
        tiles.push(Tile::from_chunk(chunk));
    }

    tiles
}

fn is_valid(size: usize, tiles_index: &Vec<Tile>, tile: &Tile) -> bool {
    if tiles_index.is_empty() {
        return true;
    }
    let pos = tiles_index.len();

    let mut left_fulfills = true;
    let mut top_fulfills = true;

    // println!("is valid???");
    // println!("{}", tile);

    if pos % size != 0 {
        let left_tile = &tiles_index[pos - 1];

        // println!(
        //     "{:?}",
        //     tiles_index
        //         .iter()
        //         .map(|tile| tile.id)
        //         .collect::<Vec<usize>>()
        // );
        // println!("left_tile");
        // println!("{}", left_tile);

        left_fulfills = left_tile.matrix[0][9] == tile.matrix[0][0]
            && left_tile.matrix[1][9] == tile.matrix[1][0]
            && left_tile.matrix[2][9] == tile.matrix[2][0]
            && left_tile.matrix[3][9] == tile.matrix[3][0]
            && left_tile.matrix[4][9] == tile.matrix[4][0]
            && left_tile.matrix[5][9] == tile.matrix[5][0]
            && left_tile.matrix[6][9] == tile.matrix[6][0]
            && left_tile.matrix[7][9] == tile.matrix[7][0]
            && left_tile.matrix[8][9] == tile.matrix[8][0]
            && left_tile.matrix[9][9] == tile.matrix[9][0];
    }

    if pos >= size {
        let top_tile = &tiles_index[pos - size];

        // println!("top_tile");
        // println!("{}", top_tile);

        top_fulfills = top_tile.matrix[9][0] == tile.matrix[0][0]
            && top_tile.matrix[9][1] == tile.matrix[0][1]
            && top_tile.matrix[9][2] == tile.matrix[0][2]
            && top_tile.matrix[9][3] == tile.matrix[0][3]
            && top_tile.matrix[9][4] == tile.matrix[0][4]
            && top_tile.matrix[9][5] == tile.matrix[0][5]
            && top_tile.matrix[9][6] == tile.matrix[0][6]
            && top_tile.matrix[9][7] == tile.matrix[0][7]
            && top_tile.matrix[9][8] == tile.matrix[0][8]
            && top_tile.matrix[9][9] == tile.matrix[0][9];
    }

    left_fulfills && top_fulfills
}

fn find_valid(
    tiles: &Vec<Tile>,
    size: usize,
    index_list: &Vec<usize>,
    tiles_index: Vec<Tile>,
    deep: usize,
) -> Option<Vec<Tile>> {
    if deep >= index_list.len() {
        return Some(tiles_index);
    }

    let index = index_list[deep];
    let tile = &tiles[index];

    for tile_variation in tile.orientations() {
        // println!("{}", tile_variation);

        if is_valid(size, &tiles_index, &tile_variation) {
            // println!("Valid!");
            let mut tiles_index = tiles_index.clone();
            tiles_index.push(tile_variation);

            let result = find_valid(tiles, size, &index_list, tiles_index, deep + 1);

            if result.is_some() {
                return result;
            }
        }
    }

    None
}

pub fn part1(input: &Vec<&str>) -> usize {
    let tiles = parse_input(input);
    let size = (tiles.len() as f64).sqrt() as usize;

    // tiles.iter().for_each(|tile| println!("{}", tile));

    for index_list in (0..tiles.len()).permutations(tiles.len()) {
        // if index_list != vec![1, 0, 8, 7, 3, 5, 6, 4, 2] {
        //     continue;
        // }

        match find_valid(&tiles, size, &index_list, vec![], 0) {
            None => {}
            Some(tiles_index) => {
                debug!(tiles_index
                    .iter()
                    .map(|tile| tile.id)
                    .collect::<Vec<usize>>());
                break;
            }
        }
    }

    0
}

// 1 0 8 7 3 5 6 4 2

// pub fn part2(input: &Vec<&str>) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 0)
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&input()), 0)
    // }
}
