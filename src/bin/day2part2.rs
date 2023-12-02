use std::fs;

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

fn min_cubes_necessary(game: Game) -> Round {
    let mut most = Round { red: 0, blue: 0, green: 0 };
    for &Round {red, green, blue} in &game.rounds {
        most.red = most.red.max(red);
        most.blue = most.blue.max(blue);
        most.green = most.green.max(green);
    }
    most
}

fn power_of_set(r: Round) -> u32 {
    let Round {red, green, blue} = r;
    red * green * blue
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
    input.lines().map(parse_line).map(min_cubes_necessary).map(power_of_set).sum()
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;
    println!("{}", parse_input(&input));
    Ok(())
}

#[test]
fn test_main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(parse_input(&input), 78111);
}