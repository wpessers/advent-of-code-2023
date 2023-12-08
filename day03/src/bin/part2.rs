use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Debug)]
enum CharType {
    Empty,
    Number(u32),
    Special(char),
}

pub fn part2(input: &str) -> String {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, character)| {
                (
                    (y as i32, x as i32),
                    match character {
                        '.' => CharType::Empty,
                        c if c.is_ascii_digit() => {
                            CharType::Number(c.to_digit(10).expect("Should be a valid number"))
                        }
                        c => CharType::Special(c),
                    },
                )
            })
        })
        .collect::<BTreeMap<(i32, i32), CharType>>();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let CharType::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers.iter_mut().last().expect("Should exist");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![((*x, *y), *num)]);
                            }
                        }
                        None => unimplemented!("Should not occur"),
                    }
                }
                None => {
                    numbers.push(vec![((*x, *y), *num)]);
                }
            }
        }
    }

    let mut total = 0;
    for symbol in map
        .iter()
        .filter(|(_key, value)| matches!(value, CharType::Special('*')))
    {
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let pos_to_check: Vec<(i32, i32)> = positions
            .iter()
            .map(|outer_pos| (outer_pos.0 + symbol.0 .1, outer_pos.1 + symbol.0 .0))
            .collect();

        let mut indexes_of_numbers = vec![];

        for pos in pos_to_check {
            for (i, num_list) in numbers.iter().enumerate() {
                if num_list.iter().any(|(num_pos, _)| num_pos == &pos) {
                    indexes_of_numbers.push(i);
                }
            }
        }

        let is_gear = indexes_of_numbers.iter().unique().count() == 2;

        if is_gear {
            total += indexes_of_numbers
                .iter()
                .unique()
                .map(|index| {
                    numbers[*index]
                        .iter()
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .product::<usize>();
        }
    }

    total.to_string()
}

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    println!("{output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", part2(input));
    }
}
