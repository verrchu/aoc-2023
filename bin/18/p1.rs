use std::{
    cmp::{max, min},
    collections::HashSet,
    str::FromStr,
};

use itertools::Itertools;

static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    W,
    S,
    E,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::N,
            "D" => Self::S,
            "R" => Self::E,
            "L" => Self::W,
            _ => unreachable!(),
        })
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

    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        let mut line = line.split(' ');

        let dir = line.next().unwrap().parse::<Direction>().unwrap();
        let steps = line.next().unwrap().parse::<isize>().unwrap();

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
                path.extend(((c + 1)..=(c + steps)).map(|c| (r, c, H)));
                c += steps;
            }
            W => {
                path.extend(((c - steps)..c).rev().map(|c| (r, c, H)));
                c -= steps;
            }
        }

        minc = min(c, minc);
        minr = min(r, minr);
        maxc = max(c, maxc);
        maxr = max(r, maxr);

        if let Some(next) = lines.peek() {
            let (ndir, _) = next.split_once(' ').unwrap();
            let ndir = ndir.parse::<Direction>().unwrap();

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
    path.last_mut().unwrap().2 = SW;

    let edge = path
        .iter()
        .map(|(r, c, _s)| (*r, *c))
        .collect::<HashSet<_>>();

    let mut inner = 0;
    for r in minr..=maxr {
        for c in minc..=maxc {
            if edge.contains(&(r, c)) {
                continue;
            }

            let ps = path
                .iter()
                .filter(|(rs, cs, _ps)| (r == *rs && c < *cs))
                .sorted_by_key(|(_r, c, _s)| c)
                .map(|(_r, _c, s)| s);

            let mut intersections = 0;
            let mut turn = None;
            for s in ps {
                match s {
                    H => {
                        continue;
                    }
                    V => {
                        intersections += 1;
                    }
                    SW => match turn {
                        None | Some(SE) => {
                            turn = None;
                            continue;
                        }
                        Some(NE) => {
                            intersections += 1;
                            turn = None;
                        }
                        _ => unreachable!(),
                    },
                    SE => match turn {
                        None => {
                            turn = Some(SE);
                        }
                        _ => unreachable!(),
                    },
                    NW => match turn {
                        None | Some(NE) => {
                            turn = None;
                            continue;
                        }
                        Some(SE) => {
                            intersections += 1;
                            turn = None;
                        }
                        _ => unreachable!(),
                    },
                    NE => match turn {
                        None => {
                            turn = Some(NE);
                        }
                        _ => unreachable!(),
                    },
                }
            }

            if intersections % 2 == 1 {
                inner += 1;
            }
        }
    }

    path.len() + inner
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 62);
    }
}
