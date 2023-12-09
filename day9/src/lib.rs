pub fn solve1(input: &str) -> isize {
    let lines: Vec<Vec<isize>> = parse(input);
    lines.iter().map(|l| predict_next(l.as_slice())).sum()
}

fn predict_next(line: &[isize]) -> isize {
    let mut next = 0;
    if line.iter().sum::<isize>() != 0 {
        let n: Vec<isize> = line.windows(2).map(|w| w[1] - w[0]).collect();
        next = predict_next(n.as_ref());
    }
    line.last().unwrap() + next
}

pub fn solve2(input: &str) -> isize {
    let lines: Vec<Vec<isize>> = parse(input);
    lines.iter().map(|l| predict_prev(l.as_slice())).sum()
}

fn predict_prev(line: &[isize]) -> isize {
    let mut prev = 0;
    if line.iter().sum::<isize>() != 0 {
        let n: Vec<isize> = line.windows(2).map(|w| w[1] - w[0]).collect();
        prev = predict_prev(n.as_ref());
    }
    line.first().unwrap() - prev
}

fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 114);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 2);
    }
}
