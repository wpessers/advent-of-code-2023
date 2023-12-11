use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn parse_nums(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("1234567890")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

fn parse_records(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(parse_nums, line_ending, parse_nums)(input)
}

// distance = (total_time - holding_time) * holding_time -> inequality: record < -h_t^2 + t*h_t <=> -h_t^2 + t*h_t - record > 0 <=> x^2 - bx + c < 0
// => x between (-b - sqrt(b^2 - 4*c)) / 2 and (-b + sqrt(b^2 - 4*c)) / 2
// t=7, d=9 -> x^2 - 7x + 9 < 0 => x between (7 - sqrt((-7)^2 - 4*1*9)) / 2 and (7 + sqrt((-7)^2 - 4*1*9)) / 2
fn part1(input: &str) -> String {
    let (_, (times, distances)) = parse_records(input).expect("Should parse");
    //dbg!(times, distances);

    times
        .into_iter()
        .zip(distances)
        .map(|(t, d)| {
            let t = t as f32;
            let d = d as f32;
            let lower = (t - (t.powf(2.0) - (4.0 * d)).sqrt()) / 2.0;
            let upper = (t + (t.powf(2.0) - (4.0 * d)).sqrt()) / 2.0;
            println!("{lower} < x < {upper}");

            let rounded_lower = lower.ceil();
            let rounded_upper = upper.floor();
            let mut nums = rounded_upper as u32 - rounded_lower as u32 + 1;
            if rounded_lower == lower {
                nums -= 1;
            }
            if rounded_upper == upper {
                nums -= 1;
            }
            nums
        })
        .product::<u32>()
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
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", part1(input));
    }
}
