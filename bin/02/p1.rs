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
            // discard "Game <ID>:" prefix since games are supplied in order anyway
            let (_prefix, rounds) = line.split_once(':').unwrap();

            rounds
                // aplit rounds
                .split(';')
                .map(|round| {
                    round
                        .trim()
                        // split round into parts where each part is "<n> <color>"
                        .split(',')
                        .map(|take| {
                            let (n, c) = take.trim().split_once(' ').unwrap();

                            (c, n.parse::<usize>().unwrap())
                        })
                        // "compress" each take into (R, G, B) tuple
                        .fold((0, 0, 0), |(r, g, b), (c, n)| match c {
                            "red" => (n, g, b),
                            "green" => (r, n, b),
                            "blue" => (r, g, n),
                            _ => unreachable!(),
                        })
                })
                // "compress" all rounds into the maximum possible (R, G, B) tuple
                .fold((0, 0, 0), {
                    |(r1, g1, b1), (r2, g2, b2)| {
                        use std::cmp::max;

                        (max(r1, r2), max(g1, g2), max(b1, b2))
                    }
                })
        })
        .enumerate()
        // filter out fames that wouldn't be possible with the given constraints
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
