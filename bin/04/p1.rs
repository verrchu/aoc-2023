use std::collections::HashSet;

// static INPUT: &str = include_str!("./example");
static INPUT: &str = include_str!("./input");

fn main() {
    let result = INPUT
        .lines()
        .filter_map(|line| {
            let (_prefix, cards) = line.split_once(':').unwrap();

            let (winning, actual) = cards.split_once('|').unwrap();

            fn parse_numbers(s: &str) -> impl Iterator<Item = usize> + '_ {
                s.trim()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<usize>().unwrap())
            }

            let winning = parse_numbers(winning);
            let actual = parse_numbers(actual).collect::<HashSet<_>>();

            let matches = winning.filter(|w| actual.contains(w)).count();

            (matches > 0).then(|| 2usize.pow((matches - 1) as u32))
        })
        .sum::<usize>();

    println!("result: {result}");
}
