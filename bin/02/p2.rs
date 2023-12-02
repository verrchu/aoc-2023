static INPUT: &str = include_str!("./example");
// static INPUT: &str = include_str!("./input");

fn main() {
    let result = INPUT
        .lines()
        .map(|line| {
            let (_prefix, rounds) = line.split_once(':').unwrap();
            rounds
                .split(';')
                .map(|round| {
                    round
                        .trim()
                        .split(',')
                        .map(|take| {
                            let (n, c) = take.trim().split_once(' ').unwrap();

                            (c, n.parse::<usize>().unwrap())
                        })
                        .fold((0, 0, 0), |(r, g, b), (c, n)| match c {
                            "red" => (n, g, b),
                            "green" => (r, n, b),
                            "blue" => (r, g, n),
                            _ => unreachable!(),
                        })
                })
                .fold((0, 0, 0), {
                    |(r1, g1, b1), (r2, g2, b2)| {
                        use std::cmp::max;

                        (max(r1, r2), max(g1, g2), max(b1, b2))
                    }
                })
        })
        .map(|(r, g, b)| r * g * b)
        .sum::<usize>();

    println!("result: {result}");
}
