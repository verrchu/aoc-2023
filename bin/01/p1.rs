use std::char;

// static INPUT: &str = include_str!("./p1-example");
static INPUT: &str = include_str!("./input");

fn main() {
    let result = INPUT
        .lines()
        .map(|line| {
            let digits = || line.chars().filter(|c| char::is_digit(*c, 10));

            let a = digits().next().unwrap().to_digit(10).unwrap();
            let b = digits().next_back().unwrap().to_digit(10).unwrap();

            a * 10 + b
        })
        .sum::<u32>();

    println!("result: {result}");
}
