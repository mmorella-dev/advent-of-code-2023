use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, fs};
use itertools::Itertools;

const INPUT_FILE: &str = "./inputs/day1.txt";

const NUMBER_WORDS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// lookup table maps words (e.g. "zero") and numbers (e.g. "0") to integers
static NUMBER_MAP: Lazy<HashMap<String, i32>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(20);
    for (x, i) in NUMBER_WORDS.iter().zip(0..) {
        map.insert(x.to_string(), i);
        map.insert(i.to_string(), i);
    }
    map
});

fn parse_digit(s: &str) -> Option<i32> {
    NUMBER_MAP.get(s).copied()
}

fn parse_input(input: &str) -> i32 {
    input.lines().map(parse_line).sum()
}

fn parse_line(line: &str) -> i32 {
    // regex which matches any number word or digit
    static REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(&NUMBER_MAP.keys().join("|"))
        .unwrap()
    });
    // iterate and match words
    let mut first = None;
    let mut last = None;
    let mut idx = 0;
    while let Some(matched) = REGEX.find_at(line, idx) {
        first = first.or(Some(matched.as_str()));
        last = Some(matched.as_str());
        // allow overlapping matches inside words, e.g. eightwo
        idx = matched.range().start + 1;
    }
    // parse matched words into
    let first_digit = parse_digit(first.unwrap()).unwrap();
    let last_digit = parse_digit(last.unwrap()).unwrap();
    return first_digit * 10 + last_digit;
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;
    println!("{}", parse_input(&input));
    Ok(())
}

#[test]
fn test_main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(parse_input(&input), 53515);
}