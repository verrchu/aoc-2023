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
    let node_re = Regex::new("[0-9A-Z]{3}").unwrap();
    for line in lines {
        let mut matches = node_re.find_iter(line);

        let node = matches.next().unwrap();
        let node_l = matches.next().unwrap();
        let node_r = matches.next().unwrap();

        let s = |m: Match| m.as_str().to_string();

        nodes.insert(s(node), (s(node_l), s(node_r)));
    }

    let start_nodes = nodes.keys().filter(|n| n.ends_with('A'));

    let steps_to_end_node = |node: &str| {
        let mut node = node;
        for (i, s) in directions.chars().cycle().enumerate() {
            if node.ends_with('Z') {
                return i;
            }

            let (node_l, node_r) = nodes.get(node).unwrap();

            match s {
                'L' => node = node_l,
                'R' => node = node_r,
                _ => unreachable!(),
            }
        }

        unreachable!()
    };

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        (a * b) / gcd(a, b)
    }

    start_nodes
        .map(|node| steps_to_end_node(node))
        .reduce(lcm)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("./p2-example");

    #[test]
    fn example() {
        assert_eq!(solution(INPUT), 6);
    }
}
