use std::collections::{hash_map::Entry, HashMap, VecDeque};

static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

struct Step {
    row: isize,
    col: isize,
    dir: Direction,
    acc: usize,
    con: usize,
}

fn solution(input: &str) -> usize {
    use Direction::*;

    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<_>>>();

    let nrow = grid.len();
    let ncol = grid.first().unwrap().len();

    let mut dp = vec![vec![HashMap::<(Direction, usize), usize>::new(); ncol]; nrow];

    let mut steps = VecDeque::from_iter([
        Step {
            row: 0,
            col: 1,
            dir: E,
            acc: 0,
            con: 1,
        },
        Step {
            row: 1,
            col: 0,
            dir: S,
            acc: 0,
            con: 1,
        },
    ]);

    while let Some(s) = steps.pop_front() {
        let inbound = s.row >= 0 && s.row < nrow as isize && s.col >= 0 && s.col < ncol as isize;
        if !inbound {
            continue;
        }

        let r = s.row as usize;
        let c = s.col as usize;

        let acc = s.acc + grid[r][c];

        let dp = dp.get_mut(r).unwrap().get_mut(c).unwrap();
        let entry = dp.entry((s.dir, s.con));

        let pass = match entry {
            Entry::Vacant(_) => true,
            Entry::Occupied(ref val) => *val.get() > acc,
        };

        if pass {
            entry.and_modify(|e| *e = acc).or_insert(acc);

            let turns = match s.dir {
                S | N => ((s.row, s.col - 1, W), (s.row, s.col + 1, E)),
                E | W => ((s.row + 1, s.col, S), (s.row - 1, s.col, N)),
            };

            for (row, col, dir) in [turns.0, turns.1] {
                steps.push_back(Step {
                    row,
                    col,
                    dir,
                    acc,
                    con: 1,
                })
            }

            if s.con < 3 {
                let (row, col) = match s.dir {
                    N => (s.row - 1, s.col),
                    S => (s.row + 1, s.col),
                    E => (s.row, s.col + 1),
                    W => (s.row, s.col - 1),
                };

                steps.push_back(Step {
                    row,
                    col,
                    dir: s.dir,
                    acc,
                    con: s.con + 1,
                })
            }
        }
    }

    *dp[nrow - 1][ncol - 1].values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 102);
    }
}
