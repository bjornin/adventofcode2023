use std::collections::HashMap;

pub fn solve1(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|l| {
            let rec = parse_line(l, 1);
            backtrack(&mut cache, &rec)
        })
        .sum()
}

pub fn solve2(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|l| {
            let rec = parse_line(l, 5);
            backtrack(&mut cache, &rec)
        })
        .sum()
}

fn parse_line(line: &str, multiplier: usize) -> Record {
    let mut split = line.trim().split_ascii_whitespace();
    let os = split.next().unwrap();
    let on = split.next().unwrap();
    let s = (0..multiplier).map(|_| os).collect::<Vec<_>>().join("?");
    let n = (0..multiplier).map(|_| on).collect::<Vec<_>>().join(",");
    let springs: Vec<char> = s.chars().collect();
    let nums: Vec<usize> = n.split(',').map(|n| n.parse::<usize>().unwrap()).collect();
    Record::new(springs, nums)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Record {
    springs: Vec<char>,
    nums: Vec<usize>,
}

impl Record {
    fn new(springs: Vec<char>, nums: Vec<usize>) -> Self {
        Self { springs, nums }
    }
}

fn backtrack(cache: &mut HashMap<Record, usize>, rec: &Record) -> usize {
    if let Some(&c) = cache.get(rec) {
        return c;
    }

    if rec.nums.is_empty() {
        match rec.springs.contains(&'#') {
            true => {
                return 0;
            }
            false => {
                return 1;
            }
        }
    }

    let groups: Vec<_> = rec
        .springs
        .split(|c| *c == '.')
        .filter_map(|g| if !g.is_empty() { Some(g.len()) } else { None })
        .collect();

    if groups == rec.nums {
        return 1;
    }

    if rec.nums.iter().sum::<usize>() + rec.nums.len() - 1 > rec.springs.len() {
        return 0;
    }

    let mut result = 0;
    if rec.springs[0] == '.' {
        result += backtrack(
            cache,
            &Record::new(rec.springs[1..].to_vec(), rec.nums.clone()),
        );
    } else if groups[0] >= rec.nums[0]
        && rec.springs.len() > rec.nums[0]
        && rec.springs[rec.nums[0]] != '#'
    {
        result += backtrack(
            cache,
            &Record::new(
                rec.springs[rec.nums[0] + 1..].to_vec(),
                rec.nums[1..].to_vec(),
            ),
        );
    }
    if rec.springs[0] == '?' {
        result += backtrack(
            cache,
            &Record::new(rec.springs[1..].to_vec(), rec.nums.clone()),
        );
    }

    cache.insert(rec.clone(), result);
    result
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_backtrack() {
        let input = vec![
            ("?? 1,1", 0),
            ("?# 1", 1),
            ("?#? 1", 1),
            ("?##? 2", 1),
            ("?? 1", 2),
            ("??? 1", 3),
            ("?? 2", 1),
            ("??? 1,1", 1),
            ("???? 1,1", 3),
            ("# 1", 1),
            ("## 2", 1),
            ("#. 1", 1),
            (".# 1", 1),
            (".? 1", 1),
            (".??. 1", 2),
            ("?..? 1", 2),
            ("???.### 1,1,3", 1),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("?##?? 2,1", 1),
            ("?###???????? 3,2,1", 10),
        ];
        let mut cache = HashMap::new();
        for (i, expected) in input {
            let rec = parse_line(i, 1);
            let result = backtrack(&mut cache, &rec);
            assert_eq!(result, expected, "backtrack: {:?}", rec);
        }
    }

    const INPUT1: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 21);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 525152);
    }
}
