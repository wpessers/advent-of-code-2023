use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    count: u32,
}

#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    subsets: Vec<Vec<Cube<'a>>>,
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    println!("{output}");
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    // X color
    let (input, (count, color)) =
        separated_pair(map_res(digit1, str::parse::<u32>), tag(" "), alpha1)(input)?;
    Ok((input, Cube { color, count }))
}

fn parse_subset(input: &str) -> IResult<&str, Vec<Cube>> {
    // "X color1, Y color2, ..."
    let (input, subset) = separated_list1(tag(", "), parse_cube)(input)?;
    Ok((input, subset))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    // "Game ", id, : , remaining (=subsets)
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, subsets) = preceded(tag(": "), separated_list1(tag("; "), parse_subset))(input)?;
    Ok((input, Game { id, subsets }))
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let min_cubes: HashMap<&str, u32> = HashMap::new();
            let game = parse_game(line).expect("Parsing should work").1;
            dbg!(&game);
            game.subsets
                .iter()
                .fold(min_cubes, |mut acc, subset| {
                    for cube in subset.iter() {
                        acc.entry(cube.color)
                            .and_modify(|count| {
                                *count = (*count).max(cube.count);
                            })
                            .or_insert(cube.count);
                    }
                    acc
                })
                .values()
                .product::<u32>()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", part2(input))
    }
}
