use std::cmp::max;

use tracing::{debug, debug_span, info_span};

static INPUT: &str = include_str!("./input");

fn main() {
    aoc_2023::setup_tracing();

    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
    let mut result = 0;

    for (i, line) in input.lines().enumerate() {
        let _span = tracing::info_span!("line", n = i).entered();

        let (springs, arrangement) = line.split_once(' ').unwrap();
        let springs = springs.chars().collect::<Vec<char>>();
        let arrangement = arrangement
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let occupy = springs.iter().filter(|c| **c == '#').count();
        let r = f(&springs, &arrangement, occupy);

        println!("{line} - {r}");

        result += r;
    }

    result
}

fn f(springs: &[char], arrangement: &[usize], occupy: usize) -> usize {
    let _span = debug_span!("arr", v = debug(arrangement)).entered();

    debug!("springs: {springs:?}");

    let Some((h, t)) = arrangement.split_first() else {
        if occupy == 0 {
            return 1;
        } else {
            return 0;
        }
    };

    if springs.len() < *h {
        return 0;
    }

    // if t.is_empty() && springs.len() == *h && springs.iter().all(|c| ['#', '?'].contains(c)) {
    //     return 1;
    // }

    let mut result = 0;
    for i in 0..(springs.len() - h + 1) {
        let candidate = &springs[i..(i + *h)];
        debug!("candidate: {candidate:?}");

        let occupy = occupy - candidate.iter().filter(|c| **c == '#').count();

        let suitable_candidate = {
            let left_bound = i == 0 || springs[i - 1] != '#';

            candidate.iter().all(|c| ['#', '?'].contains(c)) && left_bound
        };

        if suitable_candidate {
            match springs.get(i + *h) {
                Some('.' | '?') => {
                    let next = &springs[(i + *h + 1)..];
                    let next_count = f(next, t, occupy);

                    debug!("{next:?} {t:?} -> {next_count}");

                    result += next_count;
                }
                Some(_) => {
                    continue;
                }
                None => {
                    if t.is_empty() && occupy == 0 {
                        result += 1;
                    }
                }
            }
        }

        if springs[i] == '#' && result == 0 {
            return 0;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case(include_str!("./example"), 21; "main")]
    #[test_case(include_str!("./example-1"), 1; "1")]
    #[test_case(include_str!("./example-2"), 4; "2")]
    #[test_case(include_str!("./example-3"), 10; "3")]
    #[test_case(include_str!("./example-4"), 2; "4")]
    #[test_case(include_str!("./example-5"), 15; "5")]
    fn example(input: &str, value: usize) {
        assert_eq!(solution(input), value);
    }
}
