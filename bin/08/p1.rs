use std::collections::HashMap;

use regex::{Match, Regex};

static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
    let mut lines = input.lines();

    let directions = lines.next().unwrap();
    assert!(lines.next().unwrap().is_empty());

    let mut nodes = HashMap::new();
    let node_re = Regex::new("[A-Z]{3}").unwrap();
    for line in lines {
        let mut matches = node_re.find_iter(line);

        let node = matches.next().unwrap();
        let node_l = matches.next().unwrap();
        let node_r = matches.next().unwrap();

        let s = |m: Match| m.as_str().to_string();

        nodes.insert(s(node), (s(node_l), s(node_r)));
    }

    let mut steps = 0usize;
    let mut node = "AAA";
    for d in directions.trim().chars().cycle() {
        if node == "ZZZ" {
            break;
        }

        let (node_l, node_r) = nodes.get(node).unwrap();

        match d {
            'L' => node = node_l,
            'R' => node = node_r,
            _ => unreachable!(),
        }

        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("./p1-example");

    #[test]
    fn example() {
        assert_eq!(solution(INPUT), 2);
    }
}
