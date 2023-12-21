use std::collections::HashSet;

static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT, 64));
}

fn solution(input: &str, steps: usize) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let nrow = grid.len();
    let ncol = grid.first().unwrap().len();

    let mut start = None;
    for r in 0..nrow {
        for c in 0..ncol {
            if grid[r][c] == 'S' {
                start = Some((r, c));
                grid[r][c] = '.';
            }
        }
    }

    let mut state = HashSet::<(usize, usize)>::from_iter([start.unwrap()]);

    for _ in 0..steps {
        let mut next_state = HashSet::new();

        let mut add = |r: isize, c: isize| {
            let inbound = r >= 0 && c >= 0 && r < nrow as isize && c < ncol as isize;

            let r = r as usize;
            let c = c as usize;

            if grid[r][c] == '.' && inbound {
                next_state.insert((r, c));
            }
        };

        for (r, c) in state {
            let r = r as isize;
            let c = c as isize;

            add(r - 1, c);
            add(r + 1, c);
            add(r, c - 1);
            add(r, c + 1);
        }

        state = next_state;
    }

    state.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE, 6), 16);
    }
}
