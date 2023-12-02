pub fn solve1(input: &str) -> usize {
    let strings = parse(input);
    let numbers = strings.iter().map(|s| parse_number(s)).collect::<Vec<_>>();
    numbers.iter().sum()
}

pub fn solve2(input: &str) -> usize {
    let strings = parse(input);
    let numbers = strings.iter().map(|s| parse_number2(s)).collect::<Vec<_>>();
    numbers.iter().sum()
}

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

fn parse_number2(s: &str) -> usize {
    let digits = [
        "_0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "_zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    let mut first_digit_pos = s.len();
    let mut first: usize = 0;
    for (i, d) in digits.iter().enumerate() {
        if let Some(pos) = s.find(d) {
            if pos < first_digit_pos {
                first_digit_pos = pos;
                first = i % 10;
            }
        }
    }
    let mut last_digit_pos = 0;
    let mut last = 0;
    for (i, d) in digits.iter().enumerate() {
        if let Some(pos) = s.rfind(d) {
            if pos >= last_digit_pos {
                last_digit_pos = pos;
                last = i % 10;
            }
        }
    }
    first * 10 + last
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
