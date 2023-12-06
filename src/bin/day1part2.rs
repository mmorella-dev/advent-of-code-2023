use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("inputs/day1.txt")?;
    println!("{}", parse_input(&input));
    Ok(())
}

#[test]
fn test_main() {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    assert_eq!(parse_input(&input), 53515);
}

const NUMS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_digit(s: &str) -> Option<u32> {
    s.parse()
        .ok() // parsed as a number? cool, return it. otherwise...
        .or_else(|| NUMS.iter().position(|&w| w == s).map(|d| d as u32))
}

fn parse_input(input: &str) -> u32 {
    input.lines().map(parse_line).sum()
}

fn parse_line(line: &str) -> u32 {
    // regex which matches any number word or digit
    static REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"zero|one|two|three|four|five|six|seven|eight|nine|\d").unwrap());
    // iterate and match words
    let mut idx = 0;
    let mut matches = vec![];
    while let Some(matched) = REGEX.find_at(line, idx) {
        matches.push(matched.as_str());
        idx = matched.range().start + 1;
    }
    // parse into numbers
    let mut it = matches.iter().filter_map(|s| parse_digit(s)).peekable();
    let first = it.peek().unwrap().clone();
    let last = it.last().unwrap();
    // calculate 2 digit number
    return first * 10 + last;
}
