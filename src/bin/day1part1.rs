use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("./inputs/day1.txt")?;
    println!("{}", get_solution(&input));
    Ok(())
}

#[test]
fn test_main() {
    let input = fs::read_to_string("./inputs/day1.txt").unwrap();
    assert_eq!(get_solution(&input), 54388);
}

/// given the input string, return the sum of each line
fn get_solution(input: &str) -> u32 {
    input.lines().map(parse_line).sum()
}

/// get the first and last digit in a line and return a 2 digit number.
fn parse_line(l: &str) -> u32 {
    // get all the digits in the line...
    let mut iter = l.chars().filter_map(|c| c.to_digit(10)).peekable();
    // take the first and last (might be the same digit!)
    let first = *iter.peek().unwrap();
    let last = iter.last().unwrap();
    // calculate.
    first * 10 + last
}
