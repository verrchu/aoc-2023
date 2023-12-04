use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

// static INPUT: &str = include_str!("./example");
static INPUT: &str = include_str!("./input");

struct Card {
    winning: Vec<usize>,
    actual: HashSet<usize>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_prefix, cards) = s.split_once(':').unwrap();

        let (winning, actual) = cards.split_once('|').unwrap();

        fn parse_numbers(s: &str) -> impl Iterator<Item = usize> + '_ {
            s.trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().unwrap())
        }

        let winning = parse_numbers(winning).collect::<Vec<_>>();
        let actual = parse_numbers(actual).collect::<HashSet<_>>();

        Ok(Self { winning, actual })
    }
}

fn main() {
    let cards = INPUT
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect::<Vec<_>>();

    let mut queue = VecDeque::from_iter(1..=(INPUT.lines().count()));

    let mut result = 0;
    while let Some(card_id) = queue.pop_front() {
        result += 1;

        let card = cards.get(card_id - 1).unwrap();
        let matches = card
            .winning
            .iter()
            .filter(|w| card.actual.contains(w))
            .count();

        queue.extend((card_id + 1)..=(card_id + matches));
    }

    println!("result: {result}");
}
