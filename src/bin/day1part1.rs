use std::fs;

const INPUT_FILE: &str = "./inputs/day1.txt";

fn parse_input(input: &str) -> i32 {
    input.lines().map(parse_line).sum()
}

fn parse_line(l: &str) -> i32 {
    let mut iter = l.chars().filter(|c| c.is_ascii_digit()).peekable();
    let first = *iter.peek().unwrap();
    let last = iter.last().unwrap();
    let s: String = (vec![first, last]).into_iter().collect();
    s.parse().unwrap()
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;
    println!("{}", parse_input(&input));
    Ok(())
}

#[test]
fn test_main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(parse_input(&input), 54388);
}