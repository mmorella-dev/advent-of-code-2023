use itertools::Itertools;
use regex::Regex;
use std::{collections::HashSet, fs};

const INPUT_FILE: &str = "./inputs/day3.txt";
fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let grid = build_grid(&input);
    let solution = get_solution(&grid);
    println!("{}", solution);
}

#[test]
fn test_main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let grid = build_grid(&input);
    let solution = get_solution(&grid);
    assert_eq!(solution, 72246648);
}

type Grid = Vec<Vec<GridValue>>;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
enum GridValue {
    Number {
        value: u32,
        location: (usize, usize),
    },
    Gear {
        location: (usize, usize),
    },
    Empty,
}

fn build_grid(input: &str) -> Grid {
    use GridValue::*;
    let lines: Vec<_> = input.lines().collect();
    // size of the input grid
    let grid_height = lines.len();
    let grid_width = lines[0].len();
    let re = Regex::new(r"(\d+|\*)").unwrap();
    // grid of each number
    // each cell contains the starting location of the number (unique identifier), followed by its value
    let mut grid = vec![vec![Empty; grid_width]; grid_height];
    // also track locations where a gear is
    // regex scan over input, build grid
    for (i, line) in lines.iter().enumerate() {
        for mat in re.find_iter(line) {
            let j = mat.range().start;
            let text = mat.as_str();
            let location = (i, j);
            // if it's a gear...
            if text == "*" {
                grid[i][j] = Gear { location };
            } else {
                // it's a number
                let value = text.parse().unwrap();
                for j in mat.range() {
                    grid[i][j] = Number { value, location };
                }
            }
        }
    }
    grid
}

fn get_solution(grid: &Grid) -> u32 {
    use GridValue::*;
    // calculate!
    let mut sum = 0;
    for gear_location in grid
        .iter()
        .flatten()
        .filter_map(|g| match g {
            Gear { location } => Some(location),
            _ => None,
        })
        .copied()
    {
        sum += get_gear_ratio(grid, gear_location).unwrap_or(0);
    }
    sum
}

// for a given location (presumably a gear), check if there are exactly two adjacent numbers. if so, return their sum.
fn get_gear_ratio(grid: &Grid, location: (usize, usize)) -> Option<u32> {
    // iterate over a 3x3 square...
    let neighbor_numbers = get_adjacent_numbers(grid, location);
    if neighbor_numbers.len() == 2 {
        // multiply each number
        Some(neighbor_numbers.iter().product::<u32>())
    } else {
        None
    }
}
//
fn get_adjacent_numbers(grid: &Vec<Vec<GridValue>>, (r, c): (usize, usize)) -> Vec<u32> {
    use GridValue::*;
    // gets all values in a 3x3 square
    let neighbor_cells = ((r.saturating_sub(1))..=(r + 1))
        .cartesian_product((c.saturating_sub(1))..=(c + 1))
        .filter_map(|(i, j)| grid.get(i).and_then(|row| row.get(j)));
    let uniques: HashSet<_> = neighbor_cells.collect();
    let vec: Vec<_> = uniques
        .iter()
        .filter_map(|g| match g {
            Number { value, .. } => Some(value),
            _ => None,
        })
        .copied()
        .collect();
    vec
}
