use regex::Regex;

// static INPUT: &str = include_str!("./p2-example");
static INPUT: &str = include_str!("./input");

fn main() {
    static ONE: &str = "one";
    static TWO: &str = "two";
    static THREE: &str = "three";
    static FOUR: &str = "four";
    static FIVE: &str = "five";
    static SIX: &str = "six";
    static SEVEN: &str = "seven";
    static EIGHT: &str = "eight";
    static NINE: &str = "nine";

    // regex which matches a digit from 1 to 9 or a word repr of such digit
    let digit_re = {
        let words = [ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

        let pattern = format!("([1-9]|{})", words.join("|"));
        Regex::new(&pattern).unwrap()
    };

    let match_to_digit = |m: &str| match m {
        m if m == ONE => 1,
        m if m == TWO => 2,
        m if m == THREE => 3,
        m if m == FOUR => 4,
        m if m == FIVE => 5,
        m if m == SIX => 6,
        m if m == SEVEN => 7,
        m if m == EIGHT => 8,
        m if m == NINE => 9,
        m => m.chars().next().unwrap().to_digit(10).unwrap(),
    };

    let result = INPUT
        .lines()
        .map(|line| {
            // find our first digit
            let m = digit_re.find(line).unwrap();

            let a = m.as_str();
            let mut b = a;

            // we could simply use `Regex::find_iter` to find all the matches
            // and take the last one but it produces a list of _non-overlapping_
            // matches which we don't want. therefore we are looking for
            // overlapping matches by shifting offset by 1 after finding a match
            let mut offset = m.start() + 1;
            while offset < line.len() {
                if let Some(m) = digit_re.find_at(line, offset) {
                    b = m.as_str();
                    offset = m.start() + 1;
                } else {
                    break;
                }
            }

            match_to_digit(a) * 10 + match_to_digit(b)
        })
        .sum::<u32>();

    println!("result: {result}");
}
