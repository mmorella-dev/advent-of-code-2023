use std::fs;

fn main() {
    let input_path = "inputs/day6.txt";
    let input = fs::read_to_string(input_path).unwrap();
    let parsed = parse_input(&input);
    let solution = get_solution(&parsed);
    println!("{}", solution);
}

#[test]
fn test_main() {
    let input = fs::read_to_string("inputs/day6.txt").unwrap();
    let parsed = parse_input(&input);
    assert_eq!(get_solution(&parsed), 608902);
}
#[test]
fn test_sample() {
    let input = "Time:      7  15   30
    Distance:  9  40  200";
    let parsed = parse_input(&input);
    assert_eq!(get_solution(&parsed), 288);
}

struct Race {
    time: u32,
    distance: u32,
}

fn parse_input(input: &str) -> Vec<Race> {
    // read lines...
    let mut lines = input.lines();
    let times_str = lines.next().unwrap();
    let dists_str = lines.next().unwrap();
    // parse each line into numbers...
    let times = times_str.split_whitespace().filter_map(|n| n.parse().ok());
    let distances = dists_str.split_whitespace().filter_map(|n| n.parse().ok());
    // zip iterators into a vec of race objects
    let races = times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance });
    races.collect()
}

///
fn get_solution(races: &Vec<Race>) -> u32 {
    races.iter().map(calculate_race).product()
}

/// for a given race, determines how many ways there are to win it.
fn calculate_race(race: &Race) -> u32 {
    let mut wins = 0;
    for hold_time in 0..race.time {
        let time_left = race.time - hold_time;
        if hold_time * time_left > race.distance {
            wins += 1;
        }
    }
    wins
}
