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

    use crate::parser;
    use crate::network;
    
    type Solution = u32;

    pub fn solve_file(filename: &str) -> Solution {
        let input = fs::read_to_string(filename).unwrap();
        let parsed = parser::parse_input(&input);
        solve(parsed)
    }
    
    fn solve(parsed: parser::ParsedInput) -> Solution {
        let (path, network) = parsed;
        network::distance(path, network, "AAA".to_string(), "ZZZ".to_string())
    }
}

mod parser {
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
        println!("{}", line);
        let (_, [node, left, right]) = RE.captures(line).unwrap().extract();
        let key = node.into();
        let node = (left.into(), right.into());
        (key, node)
    }

    fn parse_path(line: &str) -> Path {
        line.chars().filter_map(|c| match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None
        }).collect()
    }
}

pub mod network {
    use std::collections::HashMap;
    
    pub type Key = String;
    pub type Node = (Key, Key);
    pub type Network = HashMap<Key, Node>;
    
    pub enum Direction {
        Left,
        Right
    }
    pub type Path = Vec<Direction>;

    pub fn distance(path: Path, network: Network, start: Key, end: Key) -> u32 {
        let mut current_key = &start;
        let mut steps = 0;
        for d in path.iter().cycle() {
            steps += 1;
            let node = network.get(current_key).unwrap();
            current_key = match d {
                Direction::Left => &node.0,
                Direction::Right => &node.1
            };
            if *current_key == end {
                break;
            }
        }
        steps
    }
}