use std::collections::HashMap;

use itertools::Itertools;

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
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start = chunk.next().unwrap().parse::<usize>().unwrap();
            let len = chunk.next().unwrap().parse::<usize>().unwrap();

            (start, start + len - 1)
        })
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

    let mut ranges = seeds;

    for step in STEPS {
        let step_entries = transitions.get(step).unwrap();
        let mut mapped_ranges = vec![];

        for (src, dst, len) in step_entries.iter().copied() {
            let src_range = (src, src + len - 1);
            let dst_range = (dst, dst + len - 1);

            let mut new_ranges = vec![];
            for range in ranges {
                if range.0 <= src_range.0 && range.1 >= src_range.1 {
                    mapped_ranges.push((dst_range.0, dst_range.1));

                    if range.0 < src_range.0 {
                        new_ranges.push((range.0, src_range.0 - 1));
                    }
                    if range.1 > src_range.1 {
                        new_ranges.push((src_range.1 + 1, range.1));
                    }
                } else if range.0 >= src_range.0 && range.1 <= src_range.1 {
                    mapped_ranges.push((
                        dst_range.0 + (range.0 - src_range.0),
                        dst_range.1 - (src_range.1 - range.1),
                    ));
                } else if range.0 >= src_range.0 && range.0 <= src_range.1 {
                    mapped_ranges.push((dst_range.0 + (range.0 - src_range.0), dst_range.1));

                    new_ranges.push((src_range.1 + 1, range.1));
                } else if range.1 >= src_range.0 && range.1 <= src_range.1 {
                    mapped_ranges.push((dst_range.0, dst_range.1 - (src_range.1 - range.1)));

                    new_ranges.push((range.0, src_range.0 - 1));
                } else {
                    new_ranges.push(range);
                }
            }

            ranges = new_ranges;
        }

        ranges.extend(mapped_ranges);
    }

    let result = ranges.into_iter().map(|(start, _end)| start).min().unwrap();

    println!("result: {result}");
}
