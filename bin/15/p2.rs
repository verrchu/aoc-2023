static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

fn solution(input: &str) -> usize {
    let mut boxes = vec![Vec::<(&str, usize)>::new(); 256];

    for op in input.trim().split(',') {
        if let Some(label) = op.strip_suffix('-') {
            let b = boxes.get_mut(box_index(label)).unwrap();

            if let Some(i) = b
                .iter()
                .enumerate()
                .find_map(|(i, (l, _))| (*l == label).then_some(i))
            {
                b.remove(i);
            }
        } else {
            let (label, val) = op.split_once('=').unwrap();
            let val = val.parse::<usize>().unwrap();
            let b = boxes.get_mut(box_index(label)).unwrap();

            if let Some(i) = b
                .iter()
                .enumerate()
                .find_map(|(i, (l, _))| (*l == label).then_some(i))
            {
                b[i] = (label, val);
            } else {
                b.push((label, val));
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(|(j, (_label, val))| (i + 1) * (j + 1) * val)
                .sum::<usize>()
        })
        .sum()
}

fn box_index(label: &str) -> usize {
    label.chars().fold(0, |mut acc, c| {
        acc += c as usize;
        acc *= 17;

        acc % 256
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 145);
    }
}
