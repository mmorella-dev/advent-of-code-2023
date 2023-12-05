use itertools::Itertools;
use std::{fs, ops::{Range, Add}};

const SAMPLE_FILE: &str = "./inputs/day5_sample.txt";
const INPUT_FILE: &str = "./inputs/day5.txt";
fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let (seeds, maps) = parse_input(&input);
    let solution = get_solution(&seeds, &maps);
    println!("{}", solution);
}

#[test]
fn test_main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let (seeds, maps) = parse_input(&input);
    let solution = get_solution(&seeds, &maps);
    assert_eq!(solution, 3374647);
}
#[test]
fn test_sample() {
    let input = fs::read_to_string(SAMPLE_FILE).unwrap();
    let (seeds, maps) = parse_input(&input);
    let solution = get_solution(&seeds, &maps);
    assert_eq!(solution, 35);
}

struct Map {
    range: Range<u64>, // which seeds to add
    offset: i64, // the amount to add or subtract to the location to get to the destination
}
// a set of maps for a particular layer, e.g. 
type MapLayer = Vec<Map>;

/// returns all maps, sorted from lowest to highest layer
fn parse_input(input: &str) -> (Vec<u64>, Vec<MapLayer>) {
    let mut groups = input.split("\n\n").map(|l| l.lines());
    // first line: "seeds: x x x x x"
    let seeds: Vec<_> = groups.next().unwrap().next().unwrap().strip_prefix("seeds: ").unwrap().split(" ").map(|s| s.parse().unwrap()).collect();

    let map_layers = groups.map(|mut it| {
        it.next(); // eat header
        it.map(|line| parse_map(line))
        .collect::<MapLayer>()
    }).collect_vec();

    (seeds, map_layers)
}


fn parse_map(line: &str) -> Map {
    let nums = line.split(' ').map(|n| n.parse::<u64>().unwrap()).collect_vec();
    if let [dst, src, len] = nums[..] {
        return Map {
            range: src..(src + len),
            offset: dst as i64 - (src as i64),
        };
    }
    panic!();
}

fn get_solution(seeds: &Vec<u64>, map_layers: &Vec<MapLayer>) -> u64 {
    seeds.iter()
    // .inspect(|seed| println!("{}", seed))
    .map(|&seed| get_seed_location(map_layers, seed))
    .min().unwrap()
}
fn get_seed_location(map_layers: &Vec<MapLayer>, seed: u64) -> u64 {
    let mut location = seed;
    println!("seed {}", seed);
    for layer in map_layers {
        for map in layer {
            if map.range.contains(&location) {
                println!("now {}", location);
                location = (location as i64 + map.offset) as u64;
                break;
            }
        }
    }
    location
}
