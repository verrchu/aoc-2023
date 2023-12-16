use std::collections::HashSet;

static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    W,
    S,
    E,
}

fn solution(input: &str) -> usize {
    use Direction::*;

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let nrow = grid.len();
    let ncol = grid.first().unwrap().len();

    let mut ongoing = vec![(0, 0, Direction::E)];
    let mut seen = HashSet::<(usize, usize, Direction)>::from_iter(ongoing.clone());

    while !ongoing.is_empty() {
        let mut next_ongoing = vec![];

        let mut add_next = |r: isize, c: isize, d: Direction| {
            let inbound = r >= 0 && r < nrow as isize && c >= 0 && c < ncol as isize;

            let r = r as usize;
            let c = c as usize;

            if inbound && seen.insert((r, c, d)) {
                next_ongoing.push((r, c, d));
            }
        };

        for (r, c, d) in ongoing {
            let val = grid[r][c];

            let r = r as isize;
            let c = c as isize;

            match (d, val) {
                (N, '.' | '|') => add_next(r - 1, c, N),
                (S, '.' | '|') => add_next(r + 1, c, S),
                (N | S, '-') => {
                    add_next(r, c - 1, W);
                    add_next(r, c + 1, E);
                }
                (N, '/') | (S, '\\') => add_next(r, c + 1, E),
                (N, '\\') | (S, '/') => add_next(r, c - 1, W),
                (E, '.' | '-') => add_next(r, c + 1, E),
                (W, '.' | '-') => add_next(r, c - 1, W),
                (E | W, '|') => {
                    add_next(r + 1, c, S);
                    add_next(r - 1, c, N);
                }
                (E, '/') | (W, '\\') => add_next(r - 1, c, N),
                (E, '\\') | (W, '/') => add_next(r + 1, c, S),
                _ => unreachable!(),
            }
        }

        ongoing = next_ongoing;
    }

    seen.into_iter()
        .map(|(r, c, _d)| (r, c))
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 46);
    }
}
