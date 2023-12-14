static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let nrow = grid.len();
    let ncol = grid.first().unwrap().len();

    for r in 0..nrow {
        for c in 0..ncol {
            if grid[r][c] == 'O' {
                maybe_tilt(&mut grid, r, c);
            }
        }
    }

    let mut result = 0;
    for (i, row) in grid.iter().rev().enumerate() {
        result += row.iter().filter(|c| **c == 'O').count() * (i + 1);
    }

    result
}

fn maybe_tilt(grid: &mut Vec<Vec<char>>, r: usize, c: usize) {
    if grid[r][c] != 'O' {
        return;
    }

    let free_space = (0..r).rev().take_while(|r| grid[*r][c] == '.').count();
    if free_space > 0 {
        grid[r][c] = '.';
        grid[r - free_space][c] = 'O';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 136);
    }
}
