use std::collections::HashSet;

static INPUT: &str = include_str!("./input");
// static INPUT: &str = include_str!("./example-2");

type Pattern = Vec<Vec<char>>;

fn main() {
    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
    let patterns = read_patterns(input);

    let mut result = 0;
    for (_i, mut pattern) in patterns.into_iter().enumerate() {
        let nrow = pattern.len();
        let ncol = pattern.first().unwrap().len();

        let flip = |pattern: &mut Pattern, r: usize, c: usize| match pattern[r][c] {
            '.' => pattern[r][c] = '#',
            '#' => pattern[r][c] = '.',
            _ => unreachable!(),
        };

        let h1 = find_h(&pattern).into_iter().next();
        let v1 = find_v(&pattern).into_iter().next();

        'outer: for r in 0..nrow {
            for c in 0..ncol {
                flip(&mut pattern, r, c);

                let mut h2 = find_h(&pattern);
                if !h2.is_empty() {
                    if let Some(h1) = h1.as_ref() {
                        h2.remove(h1);
                    }

                    if let Some(h2) = h2.into_iter().next() {
                        result += h2 * 100;
                        break 'outer;
                    }
                }

                let mut v2 = find_v(&pattern);
                if !v2.is_empty() {
                    if let Some(v1) = v1.as_ref() {
                        v2.remove(v1);
                    }

                    if let Some(v2) = v2.into_iter().next() {
                        result += v2;
                        break 'outer;
                    }
                }

                flip(&mut pattern, r, c);
            }
        }
    }

    result
}

fn read_patterns(input: &str) -> Vec<Pattern> {
    let mut patterns = vec![];

    let mut lines = input.lines();
    let mut buf = None;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            patterns.push(buf.take().unwrap());
            continue;
        }

        if buf.is_none() {
            buf = Some(vec![]);
        }

        buf.as_mut().unwrap().push(line);
    }

    if let Some(buf) = buf {
        patterns.push(buf);
    }

    patterns
        .into_iter()
        .map(|pattern| {
            pattern
                .into_iter()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<Vec<_>>>()
        })
        .collect()
}

fn find_h(p: &Vec<Vec<char>>) -> HashSet<usize> {
    let nrow = p.len();
    let ncol = p.first().unwrap().len();

    let sym_col = |c: usize, i: usize| (0..i).rev().zip(i..nrow).all(|(a, b)| p[a][c] == p[b][c]);

    let mut hs = HashSet::new();
    for i in 1..nrow {
        if sym_col(0, i) && (1..ncol).all(|c| sym_col(c, i)) {
            hs.insert(i);
        }
    }

    hs
}

fn find_v(p: &Vec<Vec<char>>) -> HashSet<usize> {
    let sym_row = |row: &[char], i: usize| -> bool {
        row[0..i]
            .iter()
            .rev()
            .zip(row[i..].iter())
            .all(|(a, b)| a == b)
    };

    let (h, t) = p.split_first().unwrap();
    let ncol = h.len();

    let mut vs = HashSet::new();
    for i in 1..ncol {
        if sym_row(h, i) && t.iter().all(|r| sym_row(r, i)) {
            vs.insert(i);
        }
    }

    vs
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case(include_str!("./example"), 400; "main")]
    fn example(input: &str, value: usize) {
        assert_eq!(solution(input), value);
    }
}
