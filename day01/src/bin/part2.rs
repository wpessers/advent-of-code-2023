fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    println!("{output}");
}

fn part2(input: &str) -> String {
    const WORD_REPLACEMENTS: [(&str, &str); 9] = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "4"),
        ("five", "5e"),
        ("six", "6"),
        ("seven", "7n"),
        ("eight", "e8t"),
        ("nine", "9e"),
    ];
    let res = input
        .lines()
        .map(|line| {
            let line = WORD_REPLACEMENTS
                .iter()
                .fold(line.to_string(), |acc, &(word, repl)| {
                    acc.replace(word, repl)
                });

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
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", part2(input))
    }
}
