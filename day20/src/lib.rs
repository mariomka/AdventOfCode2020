use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};

type Matrix = Vec<Vec<bool>>;

#[derive(Clone)]
struct Tile {
    id: usize,
    matrix: Matrix,
    rotation: usize,
    flip_x: bool,
    flip_y: bool,
    is_inner: bool,
    is_side: bool,
    is_corner: bool,
    matches: Vec<usize>,
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
            is_inner: false,
            is_side: false,
            is_corner: false,
            matches: vec![],
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

    fn match_horizontal(left: &Tile, right: &Tile) -> bool {
        left.matrix[0][9] == right.matrix[0][0]
            && left.matrix[1][9] == right.matrix[1][0]
            && left.matrix[2][9] == right.matrix[2][0]
            && left.matrix[3][9] == right.matrix[3][0]
            && left.matrix[4][9] == right.matrix[4][0]
            && left.matrix[5][9] == right.matrix[5][0]
            && left.matrix[6][9] == right.matrix[6][0]
            && left.matrix[7][9] == right.matrix[7][0]
            && left.matrix[8][9] == right.matrix[8][0]
            && left.matrix[9][9] == right.matrix[9][0]
    }

    fn match_vertical(top: &Tile, bottom: &Tile) -> bool {
        top.matrix[9][0] == bottom.matrix[0][0]
            && top.matrix[9][1] == bottom.matrix[0][1]
            && top.matrix[9][2] == bottom.matrix[0][2]
            && top.matrix[9][3] == bottom.matrix[0][3]
            && top.matrix[9][4] == bottom.matrix[0][4]
            && top.matrix[9][5] == bottom.matrix[0][5]
            && top.matrix[9][6] == bottom.matrix[0][6]
            && top.matrix[9][7] == bottom.matrix[0][7]
            && top.matrix[9][8] == bottom.matrix[0][8]
            && top.matrix[9][9] == bottom.matrix[0][9]
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Tile ({} x: {} y: {} inner {}) {}:",
            self.rotation, self.flip_x, self.flip_y, self.is_inner, self.id
        )?;
        writeln!(f, "Matches: {:?}", self.matches)?;
        for row in self.matrix.iter() {
            for cell in row {
                write!(f, "{}", if true == *cell { "#" } else { "." })?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Tile ({} x: {} y: {} inner {}) {}:",
            self.rotation, self.flip_x, self.flip_y, self.is_inner, self.id
        )?;
        writeln!(f, "Matches: {:?}", self.matches)?;
        for row in self.matrix.iter() {
            for cell in row {
                write!(f, "{}", if true == *cell { "#" } else { "." })?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;

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
                    self.tile.rotate();
                    self.tile.flip_x();
                }
            }
            1 => self.tile.rotate(),
            2 => self.tile.rotate(),
            3 => self.tile.rotate(),
            _ => unreachable!(),
        }

        self.index += 1;

        // 4 rotations per vertical flip are enough
        // to generate every variation
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

fn find_corners(tiles: &Vec<Tile>) -> Vec<&Tile> {
    let mut corner_tiles = Vec::new();

    for tile in tiles.iter() {
        let mut is_corner = true;

        for tile_variation in tile.orientations() {
            let mut left_count = 0;
            let mut right_count = 0;
            let mut top_count = 0;
            let mut bottom_count = 0;

            for other in tiles.iter() {
                if tile_variation.id == other.id {
                    continue;
                }

                for other_variation in other.orientations() {
                    // match left
                    if Tile::match_horizontal(&other_variation, &tile_variation) {
                        left_count += 1;
                    }

                    // match right
                    if Tile::match_horizontal(&tile_variation, &other_variation) {
                        right_count += 1;
                    }

                    // match top
                    if Tile::match_horizontal(&tile_variation, &other_variation) {
                        top_count += 1;
                    }

                    // match bottom
                    if Tile::match_horizontal(&other_variation, &tile_variation) {
                        bottom_count += 1;
                    }
                }
            }

            is_corner = is_corner && 2 == top_count + left_count + right_count + bottom_count;
        }

        if is_corner {
            corner_tiles.push(tile);
        }
    }

    corner_tiles
}

fn find_sides(tiles: &Vec<Tile>) -> Vec<Tile> {
    let mut new_tiles = Vec::new();

    for tile in tiles.iter() {
        let mut matches = HashSet::new();
        let mut left_count = 0;
        let mut right_count = 0;
        let mut top_count = 0;
        let mut bottom_count = 0;

        for tile_variation in tile.orientations() {
            for (other_index, other) in tiles.iter().enumerate() {
                if tile.id == other.id {
                    continue;
                }

                let other_variation = other;

                // match left
                if Tile::match_horizontal(&other_variation, &tile_variation) {
                    matches.insert(other_index);
                    left_count += 1;
                }

                // match right
                if Tile::match_horizontal(&tile_variation, &other_variation) {
                    matches.insert(other_index);
                    right_count += 1;
                }

                // match top
                if Tile::match_vertical(&other_variation, &tile_variation) {
                    matches.insert(other_index);
                    top_count += 1;
                }

                // match bottom
                if Tile::match_vertical(&tile_variation, &other_variation) {
                    matches.insert(other_index);
                    bottom_count += 1;
                }
            }
        }

        let count = left_count + right_count + top_count + bottom_count;
        let mut tile = tile.clone();
        tile.is_inner = count == 4;
        tile.is_side = count == 3;
        tile.is_corner = count == 2;
        tile.matches = matches.into_iter().collect();

        new_tiles.push(tile);
    }

    new_tiles
}

fn find_valid(
    tiles: &Vec<Tile>,
    size: usize,
    solution: Vec<Tile>,
    deep: usize,
) -> Option<Vec<Tile>> {
    if deep + 1 >= tiles.len() {
        return Some(solution);
    }

    let is_new_line = (deep + 1) % size == 0;

    let tile = if is_new_line {
        &solution[deep + 1 - size]
    } else {
        solution.last().unwrap()
    };

    for mtch in tile.matches.iter() {
        let match_tile = &tiles.get(*mtch).unwrap();

        for match_tile_variant in match_tile.orientations() {
            if (is_new_line && Tile::match_vertical(tile, &match_tile_variant))
                || (false == is_new_line && Tile::match_horizontal(tile, &match_tile_variant))
            {
                let mut solution = solution.clone();
                solution.push(match_tile_variant.clone());
                match find_valid(tiles, size, solution, deep + 1) {
                    None => {}
                    Some(solution) => return Some(solution),
                }
            }
        }
    }

    None
}

fn count_monsters(image: &Vec<Vec<bool>>, monster_coords: &Vec<(usize, usize)>) -> usize {
    let monster_width = 20;
    let monster_height = 3;

    let mut monsters = 0;
    let size = image.len();

    for (y, row) in image.iter().enumerate() {
        'x: for (x, _) in row.iter().enumerate() {
            if x < size - monster_width && y < size - monster_height {
                for coords in monster_coords.iter() {
                    if image[y + coords.0][x + coords.1] == false {
                        continue 'x;
                    }
                }

                monsters += 1;
            }
        }
    }

    monsters
}

fn count_cells(image: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;

    for row in image.iter() {
        for cell in row.iter() {
            if *cell == true {
                count += 1;
            }
        }
    }

    count
}

fn rotate(matrix: &Matrix, size: usize) -> Matrix {
    let mut new_matrix = vec![vec![false; size]; size];

    for (y, row) in matrix.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            new_matrix[x][(size - 1) - y] = *cell;
        }
    }

    new_matrix
}

fn flip_x(matrix: &Matrix, size: usize) -> Matrix {
    let mut new_matrix = vec![vec![false; size]; size];

    for (y, row) in matrix.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            new_matrix[y][(size - 1) - x] = *cell;
        }
    }

    new_matrix
}

pub fn part1(input: &Vec<&str>) -> usize {
    let tiles = parse_input(input);

    find_corners(&tiles)
        .iter()
        .fold(1, |num, tile| num * tile.id)
}

pub fn part2(input: &Vec<&str>) -> usize {
    let tiles = parse_input(input);
    let size = (tiles.len() as f64).sqrt() as usize;
    let tiles = find_sides(&tiles);

    let corners = tiles
        .iter()
        .filter(|tile| tile.is_corner)
        .collect::<Vec<&Tile>>();

    let mut ordered_tiles = Vec::new();

    'outer: for corner in corners {
        for corner_version in corner.orientations() {
            match find_valid(&tiles, size, vec![corner_version.clone()], 0) {
                None => {}
                Some(solution) => {
                    ordered_tiles = solution;
                    break 'outer;
                }
            }
        }
    }

    let mut image = vec![vec![false; 8 * size]; 8 * size];

    for (i_tile, tile) in ordered_tiles.iter().enumerate() {
        for (y, row) in tile.matrix.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if x > 0 && x < 9 && y > 0 && y < 9 {
                    image[y - 1 + (i_tile / size) * 8][x - 1 + (i_tile % size) * 8] = *cell;
                }
            }
        }
    }

    let monster_coords = "                  # #    ##    ##    ### #  #  #  #  #  #   "
        .chars()
        .enumerate()
        .fold(vec![], |mut coords, (index, cell)| {
            if cell == '#' {
                coords.push((index / 20, index % 20));
            }

            coords
        });

    for i in 0..8 {
        if i > 0 {
            image = rotate(&image, 8 * size);
        }

        if i == 4 {
            image = flip_x(&image, 8 * size);
        }

        let monsters = count_monsters(&image, &monster_coords);

        if monsters > 0 {
            return count_cells(&image) - monsters * monster_coords.len();
        }
    }

    0
}

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
        assert_eq!(part1(&input()), 20899048083289)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 273)
    }

    //     #[test]
    //     fn test_tile_rotation() {
    //         Tile::from_chunk(input_lines(
    //             "
    // Tile 1409:
    // ##..#.#.#.
    // ##........
    // #.#...##.#
    // #..#..#...
    // .......##.
    // ##......##
    // ..........
    // .........#
    // .#..##....
    // #.##...##.
    // ",
    //         ));
    //     }
}
