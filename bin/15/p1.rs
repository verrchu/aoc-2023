static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|val| {
            val.chars().fold(0, |mut acc, c| {
                acc += c as usize;
                acc *= 17;

                acc % 256
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 1320);
    }
}
