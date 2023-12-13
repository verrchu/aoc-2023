static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
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

    let mut result = 0;
    for (_i, pattern) in patterns.into_iter().enumerate() {
        let pattern = pattern
            .into_iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();

        let h = find_h(&pattern);
        if let Some(h) = h {
            result += 100 * h;
        }

        let v = find_v(&pattern);
        if let Some(v) = v {
            result += v;
        }
    }

    result
}

fn find_h(p: &Vec<Vec<char>>) -> Option<usize> {
    let nrow = p.len();
    let ncol = p.first().unwrap().len();

    let sym_col = |c: usize, i: usize| (0..i).rev().zip(i..nrow).all(|(a, b)| p[a][c] == p[b][c]);

    for i in 1..nrow {
        if sym_col(0, i) && (1..ncol).all(|c| sym_col(c, i)) {
            return Some(i);
        }
    }

    None
}

fn find_v(p: &Vec<Vec<char>>) -> Option<usize> {
    let sym_row = |row: &[char], i: usize| -> bool {
        row[0..i]
            .iter()
            .rev()
            .zip(row[i..].iter())
            .all(|(a, b)| a == b)
    };

    let (h, t) = p.split_first().unwrap();
    let ncol = h.len();

    for i in 1..ncol {
        if sym_row(h, i) && t.iter().all(|r| sym_row(r, i)) {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case(include_str!("./example"), 405; "main")]
    #[test_case(include_str!("./example-vertical"), 5; "vertical")]
    #[test_case(include_str!("./example-horizontal"), 400; "horizontal")]
    #[test_case(include_str!("./example-1"), 600; "1")]
    fn example(input: &str, value: usize) {
        assert_eq!(solution(input), value);
    }
}
