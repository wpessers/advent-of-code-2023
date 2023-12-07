fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> String {
    let res = input
        .lines()
        .map(|line| {
            let mut line_iter = line.chars().filter_map(|c| c.to_digit(10));
            let first = line_iter
                .next()
                .expect("Each line has to have at least one number");
            let last = line_iter.last();
            match last {
                Some(n) => format!("{first}{n}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("Invalid output, should be a number")
        })
        .sum::<u32>();
    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", part1(input))
    }
}
