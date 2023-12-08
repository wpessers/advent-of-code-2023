use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Debug)]
enum CharType {
    Empty,
    Number(u32),
    Special(char),
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{output}");
}

pub fn part1(input: &str) -> String {
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
                        None => panic!("Should not occur"),
                    }
                }
                None => {
                    numbers.push(vec![((*x, *y), *num)]);
                }
            }
        }
    }

    let mut total = 0;
    for num_list in numbers {
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
        let num_positions: Vec<(i32, i32)> = num_list.iter().map(|((y, x), _)| (*x, *y)).collect();
        let adjacent_positions: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions
                    .iter()
                    .map(|outer_pos| (outer_pos.0 + pos.1, outer_pos.1 + pos.0))
            })
            .unique()
            .filter(|num| !num_positions.contains(num))
            .collect();

        let is_part_number = adjacent_positions.iter().any(|pos| {
            let value = map.get(pos);
            if let Some(CharType::Special(_)) = value {
                true
            } else {
                false
            }
        });

        if is_part_number {
            total += num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        assert_eq!("4361", part1(input));
    }
}
