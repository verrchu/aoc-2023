use std::cmp::max;

use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    #[clap(short)]
    red: usize,
    #[clap(short)]
    green: usize,
    #[clap(short)]
    blue: usize,
}

// static INPUT: &str = include_str!("./example");
static INPUT: &str = include_str!("./input");

fn main() {
    let args = Args::parse();

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
                            "red" => (max(r, n), g, b),
                            "green" => (r, max(g, n), b),
                            "blue" => (r, g, max(b, n)),
                            _ => unreachable!(),
                        })
                })
                .fold((0, 0, 0), {
                    |(r1, g1, b1), (r2, g2, b2)| (max(r1, r2), max(g1, g2), max(b1, b2))
                })
        })
        .enumerate()
        .filter_map(|(i, (r, g, b))| {
            if r <= args.red && g <= args.green && b <= args.blue {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("result: {result}");
}
