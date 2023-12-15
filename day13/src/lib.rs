type Mirror = Vec<Vec<char>>;

pub fn solve1(input: &str) -> usize {
    let mirrors = parse(input);
    mirrors.iter().map(value).sum()
}

pub fn solve2(input: &str) -> usize {
    let mirrors = parse(input);
    mirrors.iter().map(value2).sum()
}

fn find_reflection(mirror: &Mirror) -> Option<usize> {
    (1..mirror.len())
        .find(|&r| (0..r.min(mirror.len() - r)).all(|dr| mirror[r + dr] == mirror[r - dr - 1]))
}

fn find_reflection2(mirror: &Mirror) -> Option<usize> {
    (1..mirror.len()).find(|&r| {
        let smudge: usize = (0..r.min(mirror.len() - r))
            .map(|dr| {
                mirror[r + dr]
                    .iter()
                    .zip(mirror[r - dr - 1].iter())
                    .filter(|(a, b)| a != b)
                    .count()
            })
            .sum();
        smudge == 1
    })
}

fn value(mirror: &Mirror) -> usize {
    let h = find_reflection(mirror).unwrap_or(0);
    let v = find_reflection(&transpose(mirror)).unwrap_or(0);
    100 * h + v
}

fn value2(mirror: &Mirror) -> usize {
    let h = find_reflection2(mirror).unwrap_or(0);
    let v = find_reflection2(&transpose(mirror)).unwrap_or(0);
    100 * h + v
}

fn transpose(matrix: &Mirror) -> Mirror {
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i]).collect())
        .collect()
}

fn parse(input: &str) -> Vec<Mirror> {
    input
        .trim()
        .split("\n\n")
        .map(|m| {
            m.lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn a1() {
        assert_eq!(solve1(INPUT1), 405);
    }

    #[test]
    fn a2() {
        assert_eq!(solve2(INPUT1), 400);
    }
}
