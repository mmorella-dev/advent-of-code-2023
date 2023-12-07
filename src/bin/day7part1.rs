use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day7.txt").unwrap();
    let parsed = parse_input(&input);
    let solution = get_solution(parsed);
    println!("{}", solution);
}

#[test]
fn test_main() {
    let input = fs::read_to_string("inputs/day7.txt").unwrap();
    let parsed = parse_input(&input);
    let solution = get_solution(parsed);
    assert_eq!(solution, 251121738);
}
#[test]
fn test_sample() {
    let input = fs::read_to_string("inputs/day7_sample.txt").unwrap();
    let parsed = parse_input(&input);
    let solution = get_solution(parsed);
    assert_eq!(solution, 6440);
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct Rank(u8);

#[derive(PartialOrd, PartialEq, Eq)]
struct Hand(Vec<Rank>);

type Bid = u32;

fn parse_input(input: &str) -> Vec<(Hand, Bid)> {
    input
        .lines()
        .map(|l| {
            let (h, b) = l.split_once(" ").unwrap();
            let hand: Hand = Hand(h.chars().map(|c| parse_rank(&c).unwrap()).collect());
            let bid = b.parse().unwrap();
            (hand, bid)
        })
        .collect()
}
/// returns a rank from a char, if one exists
fn parse_rank(c: &char) -> Option<Rank> {
    (match c {
        '2'..='9' => Some(c.to_digit(10).unwrap() as _),
        'T' => Some(10),
        'J' => Some(11),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None,
    })
    .and_then(|c| Some(Rank(c)))
}

fn get_solution(mut input: Vec<(Hand, Bid)>) -> u32 {
    input.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(&hand2));
    input
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1) as u32)
        .sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let by_hand_type = get_hand_type(self).cmp(&get_hand_type(other));
        by_hand_type.then_with(|| self.0.cmp(&other.0))
    }
}

fn get_hand_type(h: &Hand) -> HandType {
    use HandType::*;
    //
    let uniques: BTreeSet<_> = h.0.iter().collect();
    // how many of each card exists
    let counts: BTreeMap<_, _> = uniques
        .iter()
        .map(|&r| (r, h.0.iter().filter(|&r2| r == r2).count()))
        .collect();
    // the quantities of each card, sorted
    let mut v: Vec<_> = counts.values().collect();
    v.sort();

    match v[..] {
        [5] => FiveOfAKind,
        [1, 4] => FourOfAKind,
        [2, 3] => FullHouse,
        [1, 1, 3] => ThreeOfAKind,
        [1, 2, 2] => TwoPair,
        [1, 1, 1, 2] => OnePair,
        [1, 1, 1, 1, 1] => HighCard,
        _ => panic!("This case won't happen, assuming the input is valid."),
    }
}
