pub fn solve1(input: &str) -> usize {
    let strings = parse(input);
    let numbers = strings.iter().map(|s| parse_number(s)).collect::<Vec<_>>();
    numbers.iter().sum()
}

pub fn solve2(input: &str) -> usize {
    let strings = parse(input);
    let numbers = strings.iter().map(|s| parse_number(s)).collect::<Vec<_>>();
    numbers.iter().sum()
}

const NUMBERS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_number(s: &str) -> usize {
    let left: usize = s
        .chars()
        .find_map(|s| s.to_string().parse().ok())
        .unwrap_or(0);
    let right: usize = s
        .chars()
        .rev()
        .find_map(|s| s.to_string().parse().ok())
        .unwrap_or(0);
    left * 10 + right
}

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 142);
    }

    const INPUT2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn a2() {
        let result = solve2(INPUT2);
        assert_eq!(result, 281);
    }
}
