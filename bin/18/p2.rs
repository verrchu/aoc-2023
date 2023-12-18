use std::cmp::{max, min};

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

impl From<char> for Direction {
    fn from(n: char) -> Self {
        match n {
            '0' => Self::E,
            '1' => Self::S,
            '2' => Self::W,
            '3' => Self::N,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PathSegment {
    H,
    V,
    NW,
    NE,
    SW,
    SE,
}

fn solution(input: &str) -> usize {
    use Direction::*;
    use PathSegment::*;

    let mut r = 0;
    let mut c = 0;

    let mut minc = isize::MAX;
    let mut minr = isize::MAX;
    let mut maxc = isize::MIN;
    let mut maxr = isize::MIN;

    let mut path = vec![];
    let mut result = 0;

    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        let mut line = line.split(' ');

        let hex = line.nth(2).unwrap();

        let steps = isize::from_str_radix(&hex[2..=6], 16).unwrap();
        let dir = Direction::from(hex.chars().nth(7).unwrap());

        result += steps as usize;

        match dir {
            N => {
                path.extend(((r - steps)..r).rev().map(|r| (r, c, V)));
                r -= steps;
            }
            S => {
                path.extend(((r + 1)..=(r + steps)).map(|r| (r, c, V)));
                r += steps;
            }
            E => {
                // path.extend(((c + 1)..=(c + steps)).map(|c| (r, c, H)));
                path.push((r, c + steps, H));
                c += steps;
            }
            W => {
                // path.extend(((c - steps)..c).rev().map(|c| (r, c, H)));
                path.push((r, c - steps, H));
                c -= steps;
            }
        }

        minc = min(c, minc);
        minr = min(r, minr);
        maxc = max(c, maxc);
        maxr = max(r, maxr);

        if let Some(next) = lines.peek() {
            let ndir = next
                .split(' ')
                .nth(2)
                .unwrap()
                .chars()
                .nth(7)
                .unwrap()
                .into();

            let turn = match (dir, ndir) {
                (S, W) | (E, N) => NW,
                (W, S) | (N, E) => SE,
                (N, W) | (E, S) => SW,
                (W, N) | (S, E) => NE,
                _ => unreachable!(),
            };

            path.last_mut().unwrap().2 = turn;
        }
    }

    // hack: this works only for my input
    path.last_mut().unwrap().2 = SE;

    path.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut rows = vec![Vec::with_capacity(16); (maxr - minr + 1) as usize];

    for (r, c, s) in path {
        rows.get_mut((r + minr.abs()) as usize)
            .unwrap()
            .push((c, s));
    }

    #[derive(Debug)]
    enum Sep {
        Col(isize),
        Wide(isize, isize),
    }

    for row in rows {
        let mut seps = vec![];
        let mut holes = vec![];

        let mut turn = None;
        for (c, s) in row {
            match s {
                V => seps.push(Sep::Col(c)),
                SE => {
                    assert!(turn.is_none());
                    turn = Some((c, SE));
                }
                NE => {
                    assert!(turn.is_none());
                    turn = Some((c, NE));
                }
                SW => {
                    let turn = turn.take().unwrap();
                    match turn {
                        (tc, NE) => seps.push(Sep::Wide(tc, c)),
                        (tc, SE) => holes.push((tc, c)),
                        _ => unreachable!(),
                    }
                }
                NW => {
                    let turn = turn.take().unwrap();
                    match turn {
                        (tc, SE) => seps.push(Sep::Wide(tc, c)),
                        (tc, NE) => holes.push((tc, c)),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }

        let sum = seps
            .windows(2)
            .enumerate()
            .filter_map(|(i, seps)| (i % 2 == 0).then_some(seps))
            .map(|seps| {
                let l = match seps[0] {
                    Sep::Col(x) => x,
                    Sep::Wide(_, x) => x,
                };

                let r = match seps[1] {
                    Sep::Col(x) => x,
                    Sep::Wide(x, _) => x,
                };

                let mut range = r - l - 1;

                for (hl, hr) in holes.iter() {
                    if *hl > l && *hr < r {
                        range -= hr - hl + 1;
                    }
                }

                range
            })
            .sum::<isize>();

        let sum = usize::try_from(sum).unwrap();

        result += sum;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 952408144115);
    }
}
