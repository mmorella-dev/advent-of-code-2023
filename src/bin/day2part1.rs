use std::fs;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

const INPUT_FILE: &str = "./inputs/day2.txt";

struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

fn game_legal(game: &Game) -> bool {
    for &Round { red, green, blue } in &game.rounds {
        if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
            return false;
        }
    }
    true
}

fn parse_round(r: &str) -> Round {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for pile in r.split(',') {
        let (amount, color) = pile.trim().split_once(' ').unwrap();
        let amount: usize = amount.trim().parse().unwrap();
        match color.trim() {
            "red" => red = amount,
            "blue" => blue = amount,
            "green" => green = amount,
            _ => {}
        };
    }
    return Round { red, green, blue };
}

fn parse_game_id(l: &str) -> usize {
    let (_, i) = l.rsplit_once(' ').unwrap();
    i.parse().unwrap()
}

fn parse_line(l: &str) -> Game {
    let (game, rounds) = l.split_once(':').unwrap();
    let id = parse_game_id(game);
    let rounds: Vec<Round> = rounds.split(';').map(parse_round).collect();
    Game { id, rounds }
}

fn parse_input(input: &str) -> usize {
    input.lines().map(parse_line).filter(|g| game_legal(g)).map(|g| g.id).sum()
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;
    println!("{}", parse_input(&input));
    Ok(())
}

#[test]
fn test_main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(parse_input(&input), 2545);
}