use std::collections::HashMap;

fn hash(op: &str) -> usize {
    let b = op.bytes().fold(0, |acc, c| {
        let mut a: usize = acc;
        a += c as usize;
        a *= 17;
        a %= 256;
        a
    });
    b
}

pub fn solve1(input: &str) -> usize {
    let v = input.trim().split(',').collect::<Vec<_>>();
    v.iter().map(|&op| hash(op)).sum()
}

pub fn solve2(input: &str) -> usize {
    let v = input.trim().split(',').collect::<Vec<_>>();
    let mut boxes = HashMap::new();
    v.iter().for_each(|&op| {
        let mut s = op
            .split(|c| c == '=' || c == '-')
            .filter(|&s| !s.is_empty());
        if let Some(label) = s.next() {
            let h = hash(label);
            let b: &mut Vec<(&str, usize)> = boxes.entry(h).or_insert(vec![]);
            if let Some(focal) = s.next() {
                if let Ok(focal_num) = focal.parse::<usize>() {
                    if let Some((i, _)) = b.iter().enumerate().find(|(_, (l, _))| *l == label) {
                        b[i] = (label, focal_num);
                    } else {
                        b.push((label, focal_num));
                    }
                }
            } else if let Some((i, _)) = b.iter().enumerate().find(|(_, (l, _))| *l == label) {
                b.remove(i);
            }
        }
    });
    boxes
        .iter()
        .map(|(b, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(i, (_, f))| (1 + b) * (i + 1) * f)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    #[test]
    fn parse1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 1320);
    }
    #[test]
    fn a1() {
        let t = [
            ("rn", 0),
            ("qp", 1),
            ("rn=1", 30),
            ("cm-", 253),
            ("qp=3", 97),
            ("cm=2", 47),
            ("qp-", 14),
            ("pc=4", 180),
            ("ot=9", 9),
            ("ab=5", 197),
            ("pc-", 48),
            ("pc=6", 214),
            ("ot=7", 231),
        ];
        for (input, expected) in t.iter() {
            let result = hash(input);
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 145);
    }
}
