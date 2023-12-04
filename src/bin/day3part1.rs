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
    for (i, row) in s.iter().enumerate() {
        let groups = row
            .chars()
            .enumerate()
            .group_by(|(_, c)| c.is_ascii_digit());

        for group in groups
            .into_iter()
            .filter_map(|(k, group)| k.then_some(group))
        {
            let group: Vec<_> = group.collect();
            let has_neighbor = group.iter().any(|(j, _)| has_neighbor_symbol(s, i, *j));
            let string = group.iter().map(|(_, c)| c).collect::<String>();
            let num = string.parse::<u32>().unwrap();
            if has_neighbor {
                sum += num;
            }
            println!("{} {}", num, has_neighbor);
        }
    }
    sum
}

/// given a location, check whether any symbol is adjacent
fn has_neighbor_symbol(s: &[&str], r: usize, c: usize) -> bool {
    ((r.saturating_sub(1))..=(r + 1))
        .cartesian_product((c.saturating_sub(1))..=(c + 1))
        .filter(|(i, j)| !(*i == r && *j == c))
        .filter_map(|(i, j)| s.get(i).and_then(|row| row.chars().nth(j)))
        .any(|c| c != '.' && !c.is_ascii_digit())
}

