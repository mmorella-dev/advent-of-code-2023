use itertools::Itertools;
use std::fs;
use std::ops::Range;
use std::collections::BTreeSet;

fn main() {
    let input = fs::read_to_string("inputs/day5.txt").unwrap();
    let (seeds, maps) = parse_input(&input);
    let solution = get_solution(&seeds, &maps);
    println!("{}", solution);
    assert_eq!(solution, 6082852);
}
#[test]
fn test_sample() {
    let input = fs::read_to_string("inputs/day5_sample.txt").unwrap();
    let (seeds, maps) = parse_input(&input);
    let solution = get_solution(&seeds, &maps);
    assert_eq!(solution, 46);
}

#[derive(Clone)]
struct Map {
    range: SeedRange, // which seeds to add
    offset: i64,      // the amount to add or subtract to the location to get to the destination
}
// a set of maps for a particular layer, e.g.
type MapLayer = Vec<Map>;

type SeedRange = Range<i64>;

/// returns all maps, sorted from lowest to highest layer
fn parse_input(input: &str) -> (Vec<SeedRange>, Vec<MapLayer>) {
    let mut groups = input.split("\n\n").map(|l| l.lines());
    // first line: "seeds: x x x x x"
    let s = groups.next().unwrap().next().unwrap();
    let seed_ranges = parse_seeds(s);

    let map_layers = groups
        .map(|mut it| {
            it.next(); // eat header
            it.map(|line| parse_map(line)).collect::<MapLayer>()
        })
        .collect_vec();

    (seed_ranges, map_layers)
}

fn parse_seeds(line: &str) -> Vec<SeedRange> {
    let seed_ranges: Vec<_> = line
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .tuples::<(_, _)>()
        .map(|(start, len)| start..(start + len))
        .collect();
    seed_ranges
}

fn parse_map(line: &str) -> Map {
    let nums = line.split(' ').map(|n| n.parse().unwrap()).collect_vec();
    if let [dst, src, len] = nums[..] {
        return Map {
            range: src..(src + len),
            offset: dst as i64 - (src as i64),
        };
    }
    panic!();
}

fn get_solution(seed_ranges: &Vec<SeedRange>, map_layers: &Vec<MapLayer>) -> i64 {
    // get all the locations where the seed value is discontiguous
    let deltas = get_deltas(map_layers, seed_ranges);
    // check each location
    deltas
        .iter()
        .filter(|i| seed_ranges.iter().any(|r| r.contains(&i)))
        .map(|&seed| get_seed_location(map_layers, seed))
        .min()
        .unwrap()
}

fn get_deltas(map_layers: &Vec<Vec<Map>>, seed_ranges: &Vec<Range<i64>>) -> BTreeSet<i64> {
    use std::iter::once;

    let mut deltas = BTreeSet::<i64>::new();

    // include seed starts and ends, just in case.
    for range in seed_ranges {
        deltas.insert(range.start);
        deltas.insert(range.end);
    }
    // 
    for layer in map_layers {
        // for all existing elements, offset them by the values in this layer.
        for d in deltas.clone() {
            if let Some(map) = layer.iter().find(|m| m.range.contains(&d)) {
                deltas.insert(d + map.offset);
            }
        }
        // add all the range bounds in this layer...
        deltas.extend(layer.iter().flat_map(|m| once(m.range.start).chain(once(m.range.end - 1))).map(|d| d as i64))
    }
    deltas
}

fn get_seed_location(map_layers: &Vec<MapLayer>, seed: i64) -> i64 {
    let mut location = seed;
    for layer in map_layers {
        for map in layer {
            if map.range.contains(&location) {
                location = (location as i64 + map.offset) as i64;
                break;
            }
        }
    }
    location
}
