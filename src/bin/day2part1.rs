use std::fs;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

const INPUT_FILE: &str = "./inputs/day2.txt";

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
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
        let amount: u32 = amount.trim().parse().unwrap();
        match color.trim() {
            "red" => red = amount,
            "blue" => blue = amount,
            "green" => green = amount,
            _ => {}
        };
    }
    return Round { red, green, blue };
}

fn parse_game_id(l: &str) -> u32 {
    let (_, i) = l.rsplit_once(' ').unwrap();
    i.parse().unwrap()
}

fn parse_line(l: &str) -> Game {
    let (game, rounds) = l.split_once(':').unwrap();
    let id = parse_game_id(game);
    let rounds: Vec<Round> = rounds.split(';').map(parse_round).collect();
    Game { id, rounds }
}

fn parse_input(input: &str) -> u32 {
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