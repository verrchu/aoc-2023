use itertools::Itertools;

static INPUT: &str = include_str!("./input");

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Rank {
    _J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _T,
    _Q,
    _K,
    _A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Combination {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Combination {
    fn from_hand(cards: &[Rank]) -> Self {
        let groups = cards.into_iter().counts();
        let mut counts = groups
            .iter()
            .sorted_by(|(r1, c1), (r2, c2)| c1.cmp(c2).then_with(|| r1.cmp(r2)))
            .rev();

        let njokers = groups.get(&Rank::_J).map(ToOwned::to_owned).unwrap_or(0);

        match counts.next().unwrap() {
            (_rank, 5) => Self::FiveOfAKind,
            (Rank::_J, 4) => Self::FiveOfAKind,
            (_rank, 4) => match njokers {
                1 => Self::FiveOfAKind,
                _ => Self::FourOfAKind,
            },
            (Rank::_J, 3) => match counts.next().unwrap() {
                (_rank, 2) => Self::FiveOfAKind,
                _ => Self::FourOfAKind,
            },
            (_rank, 3) => match njokers {
                2 => Self::FiveOfAKind,
                1 => Self::FourOfAKind,
                _ => match counts.next().unwrap() {
                    (_rank, 2) => Self::FullHouse,
                    _ => Self::ThreeOfAKind,
                },
            },
            (Rank::_J, 2) => Self::ThreeOfAKind,
            (_rank, 2) => match njokers {
                2 => Self::FourOfAKind,
                1 => match counts.next().unwrap() {
                    (_rank, 2) => Self::FullHouse,
                    _ => Self::ThreeOfAKind,
                },
                _ => match counts.next().unwrap() {
                    (_rank, 2) => Self::TwoPairs,
                    _ => Self::Pair,
                },
            },
            _ => match njokers {
                1 => Self::Pair,
                _ => Self::HighCard,
            },
        }
    }
}

impl From<char> for Rank {
    fn from(c: char) -> Self {
        match c {
            '2' => Self::_2,
            '3' => Self::_3,
            '4' => Self::_4,
            '5' => Self::_5,
            '6' => Self::_6,
            '7' => Self::_7,
            '8' => Self::_8,
            '9' => Self::_9,
            'T' => Self::_T,
            'J' => Self::_J,
            'Q' => Self::_Q,
            'K' => Self::_K,
            'A' => Self::_A,
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            let hand = hand.chars().map(Into::<Rank>::into).collect::<Vec<_>>();
            let comb = Combination::from_hand(&hand);
            let bid = bid.parse::<usize>().unwrap();

            (hand, comb, bid)
        })
        .sorted_by(|(h1, c1, _), (h2, c2, _)| {
            c1.cmp(c2).then_with(|| {
                use std::cmp::Ordering;

                for (r1, r2) in h1.iter().zip_eq(h2.iter()) {
                    let ord = r1.cmp(r2);

                    if ord != Ordering::Equal {
                        return ord;
                    }
                }

                unreachable!()
            })
        })
        .enumerate()
        .fold(0, |sum, (i, (_h, _c, bid))| sum + (i + 1) * bid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        static INPUT: &str = include_str!("./example");

        assert_eq!(solution(INPUT), 5905);
    }
}
