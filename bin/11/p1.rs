use std::cmp::{max, min};

static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let nrow = grid.len();
    let ncol = grid.first().unwrap().len();

    let mut empty_rows = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            empty_rows.push(i);
        }
    }

    let mut empty_cols = Vec::new();
    for i in 0..ncol {
        if grid.iter().map(|row| row[i]).all(|c| c == '.') {
            empty_cols.push(i);
        }
    }

    let mut points = Vec::new();
    for r in 0..nrow {
        for c in 0..ncol {
            if grid[r][c] == '#' {
                points.push((r, c));
            }
        }
    }

    let mut result = 0;
    for (i, (r1, c1)) in points.iter().enumerate() {
        for (r2, c2) in points.iter().skip(i) {
            let mut steps = (r2 - r1) + c2.abs_diff(*c1);
            steps += empty_rows
                .iter()
                .skip_while(|r| *r <= r1)
                .take_while(|r| *r < r2)
                .count();
            steps += empty_cols
                .iter()
                .skip_while(|c| *c <= min(c1, c2))
                .take_while(|c| *c < max(c1, c2))
                .count();

            result += steps;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 374);
    }
}
