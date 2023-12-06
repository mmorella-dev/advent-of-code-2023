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
    time: u64,
    distance: u64,
}

fn parse_input(input: &str) -> Race {
    // read lines...
    let mut lines = input.lines();
    let times_str = lines.next().unwrap();
    let dists_str = lines.next().unwrap();
    // parse each line into numbers...
    let time = times_str.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();
    let distance = dists_str.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();
    // zip iterators into a vec of race objects
    Race { time, distance }
}

///
fn get_solution(race: &Race) -> u32 {
    calculate_race(race)
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
