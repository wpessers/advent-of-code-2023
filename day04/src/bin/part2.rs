use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space0, space1},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult, Parser,
};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug)]
struct Card {
    winners: HashSet<u32>,
    nums: HashSet<u32>,
}

impl Card {
    fn num_matches(&self) -> usize {
        self.winners.intersection(&self.nums).count()
    }
}

fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            acc
        },
    )(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    separated_pair(set, tuple((tag("|"), space1)), set)
        .map(|(winners, nums)| Card {
            winners: winners,
            nums: nums,
        })
        .parse(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, parse_card)(input)
}

pub fn part1(input: &str) -> String {
    let (_, card_data) = parse_cards(input).expect("Should parse");
    let data = card_data
        .iter()
        .map(|card| card.num_matches())
        .collect::<Vec<_>>();

    let store = (0..card_data.len())
        .map(|index| (index, 1))
        .collect::<BTreeMap<usize, u32>>();

    data.iter()
        .enumerate()
        .fold(store, |mut acc, (index, card_score)| {
            let to_add = *acc.get(&index).unwrap();

            for i in (index + 1)..(index + 1 + *card_score) {
                acc.entry(i).and_modify(|value| {
                    *value += to_add;
                });
            }
            acc
        })
        .values()
        .sum::<u32>()
        .to_string()
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", part1(input));
    }
}
