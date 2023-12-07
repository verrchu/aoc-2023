use itertools::Itertools;

static INPUT: &str = include_str!("./input");

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Rank {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _T,
    _J,
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
        let counts = cards.into_iter().counts();
        let mut counts = counts.values().sorted().rev();

        match counts.next().unwrap() {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => match counts.next().unwrap() {
                2 => Self::FullHouse,
                1 => Self::ThreeOfAKind,
                _ => unreachable!(),
            },
            2 => match counts.next().unwrap() {
                2 => Self::TwoPairs,
                1 => Self::Pair,
                _ => unreachable!(),
            },
            1 => Self::HighCard,
            _ => unreachable!(),
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

        assert_eq!(solution(INPUT), 6440);
    }
}
