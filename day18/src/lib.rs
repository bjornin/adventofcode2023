pub fn solve1(input: &str) -> usize {
    let instructions = parse(input, false);
    shoelace(instructions)
}

pub fn solve2(input: &str) -> usize {
    let instructions = parse(input, true);
    shoelace(instructions)
}

fn shoelace(instructions: Vec<(char, isize)>) -> usize {
    let vertices = instructions
        .iter()
        .fold(vec![(0, 0, 0)], |mut acc, (dir, steps)| {
            let (r, c, s) = acc.last().unwrap();
            let ns = *s + steps;
            match dir {
                'R' => acc.push((*r, c + steps, ns)),
                'L' => acc.push((*r, c - steps, ns)),
                'U' => acc.push((r - steps, *c, ns)),
                'D' => acc.push((r + steps, *c, ns)),
                _ => panic!("Unknown direction {}", dir),
            }
            acc
        });
    let s: isize = vertices
        .windows(2)
        .map(|w| {
            let (x1, y1, _) = w[0];
            let (x2, y2, _) = w[1];
            x1 * y2 - x2 * y1
        })
        .sum();
    // shoelace plus "half" of trench
    (s.abs() / 2 + 1 + vertices.last().unwrap().2 / 2) as usize
}

fn parse(input: &str, convert: bool) -> Vec<(char, isize)> {
    let v = input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let direction = parts.next().unwrap().chars().next().unwrap();
            let steps = parts.next().unwrap().parse::<isize>().unwrap();
            let color = parts
                .next()
                .unwrap()
                .chars()
                .skip(2)
                .take(6)
                .collect::<String>();
            (direction, steps, color)
        })
        .collect();
    if convert {
        convert_color(v)
    } else {
        v.iter().map(|(d, s, _)| (*d, *s)).collect()
    }
}

fn convert_color(instructions: Vec<(char, isize, String)>) -> Vec<(char, isize)> {
    let result = instructions
        .iter()
        .map(|(_, _, color)| {
            let s = color.split_at(5);
            let dist = isize::from_str_radix(s.0, 16).unwrap();
            let dir = s.1.parse().unwrap();
            let d = match dir {
                0 => 'R',
                1 => 'D',
                2 => 'L',
                3 => 'U',
                _ => panic!("Unknown direction {}", dir),
            };
            (d, dist)
        })
        .collect::<Vec<_>>();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r"
    R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)
    ";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 62);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 952408144115);
    }
}
