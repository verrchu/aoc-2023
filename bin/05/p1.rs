use std::collections::HashMap;

// static INPUT: &str = include_str!("./example");
static INPUT: &str = include_str!("./input");

static STEPS: &[&str] = &[
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

fn main() {
    let mut lines = INPUT.lines();

    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|id| id.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    assert!(lines.next().unwrap().is_empty());

    let mut transitions = HashMap::<&'static str, Vec<(usize, usize, usize)>>::new();

    for step in STEPS {
        let headline = lines.next().unwrap();
        assert!(headline.strip_prefix(step).is_some());

        let mut step_entires = vec![];

        loop {
            let Some(line) = lines.next() else {
                break;
            };
            if line.is_empty() {
                break;
            }

            let mut numbers = line.split(' ').map(|n| n.parse::<usize>().unwrap());
            let dst_range_start = numbers.next().unwrap();
            let src_range_start = numbers.next().unwrap();
            let range_len = numbers.next().unwrap();

            assert!(numbers.next().is_none());

            step_entires.push((src_range_start, dst_range_start, range_len));
        }

        assert!(transitions.insert(step, step_entires).is_none());
    }

    let mut min_location = usize::MAX;
    for seed in seeds.iter().copied() {
        let mut id = seed;
        for step in STEPS {
            let step_entries = transitions.get(step).unwrap();
            for (src, dst, len) in step_entries.iter().copied() {
                if (src..(src + len)).contains(&id) {
                    id = dst + (id - src);
                    break;
                }
            }
        }

        min_location = std::cmp::min(min_location, id);
    }

    println!("result: {min_location}");
}
