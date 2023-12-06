use itertools::Itertools;

// static INPUT: &str = include_str!("./example");
// static INPUT: &str = include_str!("./p1-input");
static INPUT: &str = include_str!("./p2-input");

fn main() {
    let mut lines = INPUT.lines();

    let mut next_line = |prefix: &str| lines.next().unwrap().strip_prefix(prefix).unwrap();

    fn parse_numbers(s: &str) -> impl Iterator<Item = isize> + '_ {
        s.split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<isize>().unwrap())
    }

    let measures = parse_numbers(next_line("Time:")).zip_eq(parse_numbers(next_line("Distance:")));

    let mut result = 1;
    for (t, d) in measures {
        // d = x(t - x)
        let discriminant = (t.pow(2) - 4 * -1 * -(d + 1)) as f64;

        let root1 = (-t as f64 + discriminant.sqrt()) / -2.0;
        let root2 = (-t as f64 - discriminant.sqrt()) / -2.0;

        let solutions = root2.floor() + 1.0 - root1.ceil();

        result *= solutions as usize;
    }

    println!("result: {result}");
}
