use itertools::*;
use rayon::prelude::*;

const TILE_HEIGHT: usize = 3;
const TILE_WIDTH: usize = 3;

type Tile = [u8; TILE_HEIGHT];

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
        .par_iter_mut()
        .filter_map(|(grid, tile_counts)| backtrack_init(&tiles, grid, tile_counts).then_some(()))
        .count()
        .to_string()
}

pub fn part_2(_input: &str) -> String {
    "Merry Christmas!".to_string()
}
