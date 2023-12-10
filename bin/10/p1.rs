static INPUT: &str = include_str!("./input");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn main() {
    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let start = find_start(&grid);

    let (mut pos, mut dir) = find_connection(&grid, start);

    let mut steps = 0;
    loop {
        steps += 1;

        if pos == start {
            break;
        }

        let p = grid[pos.0][pos.1];

        use Direction::*;
        match (dir, p) {
            (S, '|') => {
                pos = (pos.0 + 1, pos.1);
            }
            (S, 'L') => {
                pos = (pos.0, pos.1 + 1);
                dir = E;
            }
            (S, 'J') => {
                pos = (pos.0, pos.1 - 1);
                dir = W;
            }
            (N, '|') => {
                pos = (pos.0 - 1, pos.1);
            }
            (N, 'F') => {
                pos = (pos.0, pos.1 + 1);
                dir = E;
            }
            (N, '7') => {
                pos = (pos.0, pos.1 - 1);
                dir = W;
            }
            (E, '-') => {
                pos = (pos.0, pos.1 + 1);
            }
            (E, 'J') => {
                pos = (pos.0 - 1, pos.1);
                dir = N;
            }
            (E, '7') => {
                pos = (pos.0 + 1, pos.1);
                dir = S;
            }
            (W, '-') => {
                pos = (pos.0, pos.1 - 1);
            }
            (W, 'L') => {
                pos = (pos.0 - 1, pos.1);
                dir = N;
            }
            (W, 'F') => {
                pos = (pos.0 + 1, pos.1);
                dir = S;
            }
            _ => unreachable!(),
        }
    }

    steps / 2
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut start = None;
    for (i, r) in grid.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == 'S' {
                start = Some((i, j));
                break;
            }
        }
    }

    start.unwrap()
}

fn find_connection(grid: &Vec<Vec<char>>, (r, c): (usize, usize)) -> ((usize, usize), Direction) {
    let nrow = grid.len();
    let ncol = grid.first().map(|row| row.len()).unwrap();

    if c > 0 {
        let x = grid[r][c - 1];
        if ['F', '-', 'L'].contains(&x) {
            return ((r, c - 1), Direction::W);
        }
    }

    if r > 0 {
        let x = grid[r - 1][c];
        if ['F', '|', '7'].contains(&x) {
            return ((r - 1, c), Direction::N);
        }
    }

    if c < ncol - 1 {
        let x = grid[r][c + 1];
        if ['J', '-', '7'].contains(&x) {
            return ((r, c + 1), Direction::E);
        }
    }

    if r < nrow - 1 {
        let x = grid[r + 1][c];
        if ['L', '|', 'J'].contains(&x) {
            return ((r + 1, c), Direction::S);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case(include_str!("./p1-example-1"), 4; "1")]
    #[test_case(include_str!("./p1-example-2"), 8; "2")]
    fn example(input: &str, steps: usize) {
        assert_eq!(solution(input), steps);
    }
}
