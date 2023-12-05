use std::char;

// static INPUT: &str = include_str!("./example");
static INPUT: &str = include_str!("./input");

fn main() {
    let mut schema = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let nrows = schema.len();
    let ncols = schema[0].len();

    let extract_number = |schema: &mut Vec<Vec<char>>, r: isize, c: isize| -> Option<usize> {
        if r >= nrows as isize || c >= ncols as isize {
            return None;
        }

        let r = usize::try_from(r).unwrap();
        let c = usize::try_from(c).unwrap();

        let is_digit = |r: usize, c: usize| -> bool { char::is_digit(schema[r][c], 10) };

        if is_digit(r, c) {
            let mut lc = c as isize;
            while lc > 0 && is_digit(r, (lc - 1) as usize) {
                lc -= 1;
            }

            let mut rc = c;
            while rc + 1 < ncols && is_digit(r, rc + 1) {
                rc += 1;
            }

            let number = schema[r][(lc as usize)..=rc]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            for c in (lc as usize)..=rc {
                schema[r][c] = '.';
            }

            return Some(number);
        }

        None
    };

    let extract_surrounding_numbers =
        |schema: &mut Vec<Vec<char>>, r: usize, c: usize| -> Vec<usize> {
            let mut numbers = vec![];

            let r = r as isize;
            let c = c as isize;

            if let Some(n) = extract_number(schema, r - 1, c - 1) {
                numbers.push(n)
            }
            if let Some(n) = extract_number(schema, r - 1, c) {
                numbers.push(n)
            }
            if let Some(n) = extract_number(schema, r - 1, c + 1) {
                numbers.push(n)
            }
            if let Some(n) = extract_number(schema, r, c - 1) {
                numbers.push(n)
            }
            if let Some(n) = extract_number(schema, r, c + 1) {
                numbers.push(n)
            }
            if let Some(n) = extract_number(schema, r + 1, c - 1) {
                numbers.push(n)
            }
            if let Some(n) = extract_number(schema, r + 1, c) {
                numbers.push(n)
            }
            if let Some(n) = extract_number(schema, r + 1, c + 1) {
                numbers.push(n)
            }

            numbers
        };

    let mut result = 0;
    for r in 0..nrows {
        for c in 0..ncols {
            let val = schema[r][c];
            if val == '*' {
                let numbers = extract_surrounding_numbers(&mut schema, r, c);
                if numbers.len() == 2 {
                    result += numbers[0] * numbers[1];
                }
            }
        }
    }

    println!("result: {result}");
}
