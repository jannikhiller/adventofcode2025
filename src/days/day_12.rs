use itertools::*;

const TILE_HEIGHT: usize = 3;
const TILE_WIDTH: usize = 3;

type Tile = [u8; TILE_HEIGHT];

#[derive(Clone)]
struct Grid {
    rows: usize,
    cols: usize,
    _data: Vec<u64>,
}

fn count_ones<T: num_traits::int::PrimInt>(int_array: &[T]) -> usize {
    int_array
        .iter()
        .map(|row_int| row_int.count_ones())
        .sum::<u32>() as usize
}

impl Grid {
    fn tile_fits(&self, tile: &Tile, (row, col): (usize, usize)) -> bool {
        izip!(tile, &self._data[row..])
            .all(|(tile_row, grid_row)| (*tile_row as u64) << col & grid_row == 0)
    }

    fn xor_tile(&mut self, tile: &Tile, (row, col): (usize, usize)) {
        for (tile_row, grid_row) in izip!(tile, &mut self._data[row..]) {
            *grid_row ^= (*tile_row as u64) << col
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row_int in &self._data {
            for col in 0..self.cols {
                if row_int & (1 << col) != 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn compute_available_cells(&self, row: usize) -> usize {
        assert!(row <= self.rows - 3);
        // compute number of empty cells starting in row
        (self.rows - row) * self.cols - count_ones(&self._data[row..row + 3])
    }
}

fn backtrack_init(tiles: &[[Tile; 4]], grid: &mut Grid, tile_counts: &mut [usize]) -> bool {
    let tile_sizes: Vec<usize> = tiles.iter().map(|t| count_ones(&t[0])).collect();

    for (i, tile) in tiles.iter().enumerate() {
        if tile_counts[i] == 0 {
            continue;
        }
        tile_counts[i] -= 1;
        // only consider one 90° rotation as the square grid has a 180° symmetry
        for rotation in &tile[..2] {
            grid.xor_tile(rotation, (0, 0));
            if backtrack(tiles, &tile_sizes, grid, tile_counts, (0, 1)) {
                return true;
            }
            grid.xor_tile(rotation, (0, 0));
        }
        tile_counts[i] += 1;
    }
    backtrack(tiles, &tile_sizes, grid, tile_counts, (0, 1))
}

fn backtrack(
    tiles: &[[Tile; 4]],
    tile_sizes: &[usize],
    grid: &mut Grid,
    tile_counts: &mut [usize],
    (row, col): (usize, usize),
) -> bool {
    if tile_counts.iter().all(|&tc| tc == 0) {
        return true;
    }
    if row > grid.rows - TILE_HEIGHT {
        return false;
    }

    let required_cells = izip!(tile_sizes, tile_counts.iter())
        .map(|(ts, tc)| *ts * *tc)
        .sum::<usize>();

    if grid.compute_available_cells(row) < required_cells {
        return false;
    }
    if col > grid.cols - TILE_WIDTH {
        return backtrack(tiles, tile_sizes, grid, tile_counts, (row + 1, 0));
    }

    for (i, tile) in tiles.iter().enumerate() {
        if tile_counts[i] == 0 {
            continue;
        }
        tile_counts[i] -= 1;
        for rotation in tile {
            if !grid.tile_fits(rotation, (row, col)) {
                continue;
            }
            grid.xor_tile(rotation, (row, col));
            if backtrack(tiles, tile_sizes, grid, tile_counts, (row, col)) {
                return true;
            }
            grid.xor_tile(rotation, (row, col));
        }
        tile_counts[i] += 1;
    }
    backtrack(tiles, tile_sizes, grid, tile_counts, (row, col + 1))
}

#[allow(clippy::needless_range_loop)]
fn rotate_tile(tile: &Tile) -> Tile {
    const { assert!(TILE_HEIGHT == 3 && TILE_WIDTH == 3) }
    let mut new_tile = [0; TILE_HEIGHT];
    for row in 0..TILE_HEIGHT {
        for col in 0..TILE_WIDTH {
            if tile[row] & (1 << col) != 0 {
                new_tile[col] |= 1 << (2 - row)
            }
        }
    }
    new_tile
}

#[allow(dead_code)]
fn print_tile(tile: &Tile) {
    for row_int in tile {
        for col in 0..TILE_WIDTH {
            if row_int & (1 << col) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse_tile(tile_str: &str) -> [Tile; 4] {
    let mut tile = [0; TILE_HEIGHT];

    for (row, row_str) in tile_str.lines().skip(1).enumerate() {
        for (col, c) in row_str.chars().enumerate() {
            if c == '#' {
                tile[row] |= 1 << col;
            }
        }
    }

    repeat_n((), 4)
        .map(|_| {
            tile = rotate_tile(&tile);
            tile
        })
        .collect_array()
        .unwrap()
}

type Problem = (Grid, Vec<usize>);
fn parse_input(input: &str) -> (Vec<[Tile; 4]>, Vec<Problem>) {
    let groups: Vec<_> = input.split("\n\n").collect();

    let tiles = groups[..groups.len() - 1]
        .iter()
        .copied()
        .map(parse_tile)
        .collect();

    let problems = groups
        .last()
        .unwrap()
        .lines()
        .map(|s| {
            let mut it = s.split_whitespace();
            let dimension_str = it.next().unwrap();
            let (rows, cols) = dimension_str
                .trim_end_matches(":")
                .split("x")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let tile_counts = it.map(|n| n.parse().unwrap()).collect();
            (
                Grid {
                    rows,
                    cols,
                    _data: vec![0; rows],
                },
                tile_counts,
            )
        })
        .collect();

    (tiles, problems)
}

pub fn part_1(input: &str) -> String {
    let (tiles, mut problems) = parse_input(input);

    problems
        .iter_mut()
        .filter_map(|(grid, tile_counts)| backtrack_init(&tiles, grid, tile_counts).then_some(()))
        .count()
        .to_string()
}

pub fn part_2(_input: &str) -> String {
    "Merry Christmas!".to_string()
}

// generated by ai
#[cfg(test)]
mod tests {
    use super::*;

    // Replicating your helper for easy grid creation
    fn create_grid(rows: usize, cols: usize, pattern: &[&str]) -> Grid {
        let mut data = Vec::with_capacity(rows);
        for r in 0..rows {
            let mut row_int = 0u64;
            if r < pattern.len() {
                for (c, char) in pattern[r].chars().enumerate() {
                    if char == '#' {
                        row_int |= 1 << c;
                    }
                }
            }
            data.push(row_int);
        }
        Grid {
            rows,
            cols,
            _data: data,
        }
    }

    #[test]
    fn test_small_regions_are_ignored() {
        let grid = create_grid(3, 3, &["...", "..#", "###"]);
        assert_eq!(grid.compute_available_cells(), 0);
    }

    #[test]
    fn test_exact_min_size_region() {
        let grid = create_grid(3, 3, &["...", "...", "###"]);
        assert_eq!(grid.compute_available_cells(), 6);
    }

    #[test]
    fn test_mixed_regions() {
        let grid = create_grid(4, 6, &["######", "..#...", "..#...", "######"]);

        assert_eq!(grid.compute_available_cells(), 6);
    }

    #[test]
    fn test_large_snake_connectivity() {
        let grid = create_grid(2, 5, &[".....", "....."]);
        assert_eq!(grid.compute_available_cells(), 10);
    }

    #[test]
    fn test_two_valid_regions() {
        let grid = create_grid(3, 7, &[".......", "#######", "......#"]);

        assert_eq!(grid.compute_available_cells(), 13);
    }

    #[test]
    fn test_almost_valid_regions() {
        let grid = create_grid(3, 5, &[".....", "#####", "....."]);
        assert_eq!(grid.compute_available_cells(), 0);
    }
}
