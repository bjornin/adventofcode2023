use std::collections::HashMap;

pub fn solve1(input: &str) -> u32 {
    let v: Vec<String> = parse(input);
    v.iter()
        .map(|c| {
            let (_, winning, played) = get_parts(c);
            let point = winning.iter().fold(0, |mut acc, n| {
                if played.contains(n) {
                    if acc == 0 {
                        acc = 1;
                    } else {
                        acc *= 2;
                    }
                }
                acc
            });
            point
        })
        .sum()
}

fn get_parts(c: &str) -> (usize, Vec<usize>, Vec<usize>) {
    let mut parts = c.split(|d| d == ':' || d == '|');
    let card = parts
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse()
        .unwrap();
    let winning = parts
        .next()
        .unwrap_or("")
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let played = parts
        .next()
        .unwrap_or("")
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    (card, winning, played)
}

pub fn solve2(input: &str) -> usize {
    let v: Vec<String> = parse(input);
    let mut multiplier: HashMap<usize, usize> = HashMap::new();
    v.iter().for_each(|c| {
        let (card, winning, played) = get_parts(c);
        let point = winning.iter().fold(0, |mut acc: usize, n| {
            if played.contains(n) {
                acc += 1;
            }
            acc
        });
        let current_multi = *multiplier.entry(card).and_modify(|e| *e += 1).or_insert(1);
        for i in 0..point {
            multiplier
                .entry(card + i + 1)
                .and_modify(|e| *e += current_multi)
                .or_insert(current_multi);
        }
    });
    multiplier.values().sum()
}

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 13);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 30);
    }
}
