use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Card {
    raw: String,
    typ: CardType,
    bid: u16,
}

#[aoc_runner::timeit]
fn main() -> usize {
    let card_strength = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ]
    .iter()
    .rev()
    .enumerate()
    .map(|(idx, ch)| (*ch, idx as u8))
    .collect::<HashMap<char, u8>>();

    let lines = aoc::read_input_lines(7);
    let mut cards = lines
        .iter()
        .map(|input| {
            let mut iter = input.split_whitespace();
            Card {
                raw: input.clone(),
                typ: parse_card_type(iter.next().unwrap()),
                bid: iter.next().unwrap_or("0").parse::<u16>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    cards.sort_by(|a, b| {
        if a.typ != b.typ {
            a.typ.cmp(&b.typ)
        } else {
            for idx in 0..a.raw.len() {
                let (x, y) = (
                    card_strength.get(&a.raw.chars().nth(idx).unwrap()).unwrap(),
                    card_strength.get(&b.raw.chars().nth(idx).unwrap()).unwrap(),
                );

                if x != y {
                    return x.cmp(y);
                }
            }

            Ordering::Equal
        }
    });

    cards
        .iter()
        .enumerate()
        .map(|(idx, card)| card.bid as usize * (idx + 1))
        .sum()
}

fn parse_card_type(input: &str) -> CardType {
    let mut map = HashMap::new();
    for c in input.chars() {
        *map.entry(c).or_insert(0) += 1_u8;
    }

    match (map.values().max().unwrap(), map.len()) {
        (5, 1) => CardType::FiveOfAKind,
        (4, 2) => CardType::FourOfAKind,
        (3, 2) => CardType::FullHouse,
        (3, 3) => CardType::ThreeOfAKind,
        (2, 3) => CardType::TwoPair,
        (2, 4) => CardType::OnePair,
        _ => CardType::HighCard,
    }
}
