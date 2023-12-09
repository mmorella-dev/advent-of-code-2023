fn main() {
    println!("{}", problem::solve_file("inputs/day8.txt"));
}

#[test]
fn test_main() {
    assert_eq!(problem::solve_file("inputs/day8.txt"), 11567);
}
#[test]
fn test_sample() {
    assert_eq!(problem::solve_file("inputs/day8_sample.txt"), 6);
}

mod problem {
    use std::fs;

    use crate::network;
    use crate::parser;

    type Solution = u64;

    pub fn solve_file(filename: &str) -> Solution {
        let input = fs::read_to_string(filename).unwrap();
        let parsed = parser::parse_input(&input);
        solve(parsed)
    }

    fn solve(parsed: parser::ParsedInput) -> Solution {
        let (path, network) = parsed;
        network::distance_ghost(&path, &network)
    }
}

mod parser {
    use itertools::Itertools;
    use once_cell::sync::Lazy;
    use regex::Regex;

    use crate::network::*;

    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap());

    pub type ParsedInput = (Path, Network);

    pub fn parse_input(input: &str) -> ParsedInput {
        let mut lines = input.lines();
        let path_str = lines.next().unwrap();
        let path = parse_path(path_str);
        lines.next(); // <- skip newline
        let network = lines.map(parse_network_node).collect();
        (path, network)
    }

    fn parse_network_node(line: &str) -> (Key, Node) {
        let (_, [node, left, right]) = RE.captures(line).unwrap().extract();
        let key = node.chars().next_tuple().unwrap();
        let node = (
            left.chars().next_tuple().unwrap(),
            right.chars().next_tuple().unwrap(),
        );
        (key, node)
    }

    fn parse_path(line: &str) -> Path {
        line.chars()
            .filter_map(|c| match c {
                'L' => Some(Direction::Left),
                'R' => Some(Direction::Right),
                _ => None,
            })
            .collect()
    }
}

pub mod network {
    use std::collections::HashMap;

    use itertools::Itertools;
    use num::integer::lcm;

    pub type Key = (char, char, char);
    pub type Node = (Key, Key);
    pub type Network = HashMap<Key, Node>;

    #[derive(Clone, Copy)]
    pub enum Direction {
        Left,
        Right,
    }
    pub type Path = Vec<Direction>;

    pub fn get_next_step<'a>(network: &'a Network, key: &'a Key, dir: Direction) -> &'a Key {
        let node = network.get(key).unwrap();
        match dir {
            Direction::Left => &node.0,
            Direction::Right => &node.1,
        }
    }
    /// the minimal distance from an A node to a Z node.
    pub fn distance_to_z<'a>(path: &Path, network: &'a Network, key: &'a Key) -> usize {
        let mut steps = 0;
        let mut current_key = key;
        for dir in path.iter().copied().cycle() {
            steps += 1;
            current_key = get_next_step(network, current_key, dir);
            if current_key.2 == 'Z' {
                break;
            }
        }
        steps
    }

    pub fn distance_ghost(path: &Path, network: &Network) -> u64 {
        let mut current_keys = network.keys().filter(|k| k.2 == 'A').collect_vec();

        let mut path_cycle = path.iter().copied().cycle();
        let mut steps = 0;

        // cycle_count
        current_keys
            .iter()
            .map(|k| distance_to_z(path, network, k))
            .reduce(|s1, s2| lcm(s1, s2))
            .unwrap()
            .try_into().unwrap()
    }
}
