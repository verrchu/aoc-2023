static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> isize {
    let mut result = 0;
    for line in input.lines() {
        let mut numbers = line
            .split(' ')
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        loop {
            if numbers.iter().all(|n| *n == 0) {
                break;
            }

            result += numbers.last().unwrap();
            numbers = numbers.windows(2).map(|w| w[1] - w[0]).collect();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 114);
    }
}
