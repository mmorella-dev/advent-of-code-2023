use itertools::Itertools;
use std::fs;

const INPUT_FILE: &str = "./inputs/day3.txt";
fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let parsed = parse_input(&input);
    let solution = get_solution(&parsed);
    println!("{}", solution);
}

#[test]

fn test_main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let parsed = parse_input(&input);
    let solution = get_solution(&parsed);
    assert_eq!(solution, 498559);
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn get_solution(s: &[&str]) -> u32 {
    let mut sum = 0;
    // for each row...
    s.iter().enumerate().map(|(i, row)| {
        // split into groups of digits and non-digits
        let groups = row
            .chars()
            .enumerate()
            .group_by(|(_, c)| c.is_ascii_digit());
        // filter to just only digits
        let digit_groups = groups
            .into_iter()
            .filter_map(|(k, group)| k.then_some(group));
        // for each set of digits...
        return digit_groups
            .map(|group| group.collect::<Vec<_>>())
            .filter(|group| group.iter().any(|(j, _)| has_neighbor_symbol(s, i, *j)))
            .map(|group| {
                group
                    .iter()
                    .map(|(_, c)| c.to_digit(10).unwrap())
                    .reduce(|acc, d| acc * 10 + d)
                    .unwrap()
            })
            .sum::<u32>();
    }).sum()
}

/// given a location, check whether any symbol is adjacent
fn has_neighbor_symbol(s: &[&str], r: usize, c: usize) -> bool {
    ((r.saturating_sub(1))..=(r + 1))
        .cartesian_product((c.saturating_sub(1))..=(c + 1))
        .filter(|(i, j)| !(*i == r && *j == c))
        .filter_map(|(i, j)| s.get(i).and_then(|row| row.chars().nth(j)))
        .any(|c| c != '.' && !c.is_ascii_digit())
}
