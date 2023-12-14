use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut hashes = HashMap::new();
    let mut i = 0;
    let (start, step) = loop {
        i += 1;

        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        let mut hasher = DefaultHasher::new();
        grid.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some(start) = hashes.insert(hash, i) {
            break (start, i - start);
        }
    };

    let steps_left = (1_000_000_000 - start) % step;

    for _ in 0..steps_left {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
    }

    let mut result = 0;
    for (i, row) in grid.iter().rev().enumerate() {
        result += row.iter().filter(|c| **c == 'O').count() * (i + 1);
    }

    result
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    let nrow = grid.len();
    let ncol = grid.first().unwrap().len();

    for r in 0..nrow {
        for c in 0..ncol {
            if grid[r][c] == 'O' {
                let free_space = (0..r).rev().take_while(|r| grid[*r][c] == '.').count();
                if free_space > 0 {
                    grid[r][c] = '.';
                    grid[r - free_space][c] = 'O';
                }
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    let nrow = grid.len();
    let ncol = grid.first().unwrap().len();

    for r in (0..nrow).rev() {
        for c in 0..ncol {
            if grid[r][c] == 'O' {
                let free_space = ((r + 1)..nrow).take_while(|r| grid[*r][c] == '.').count();
                if free_space > 0 {
                    grid[r][c] = '.';
                    grid[r + free_space][c] = 'O';
                }
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    let nrow = grid.len();
    let ncol = grid.first().unwrap().len();

    for r in 0..nrow {
        for c in 0..ncol {
            if grid[r][c] == 'O' {
                let free_space = (0..c).rev().take_while(|c| grid[r][*c] == '.').count();
                if free_space > 0 {
                    grid[r][c] = '.';
                    grid[r][c - free_space] = 'O';
                }
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    let nrow = grid.len();
    let ncol = grid.first().unwrap().len();

    for r in 0..nrow {
        for c in (0..ncol).rev() {
            if grid[r][c] == 'O' {
                let free_space = ((c + 1)..ncol).take_while(|c| grid[r][*c] == '.').count();
                if free_space > 0 {
                    grid[r][c] = '.';
                    grid[r][c + free_space] = 'O';
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 64);
    }
}
