use std::collections::HashMap;

pub fn solve1(input: &str) -> usize {
    let (line_length, v) = parse(input);
    let numbers = get_number_map(&v);
    numbers
        .iter()
        .filter(|(&pos, &number)| {
            let number_length = number.to_string().len();
            let check_positions = get_neighbor_pos(pos, number_length, line_length, &v);
            check_positions
                .iter()
                .any(|&pos| check_sign(v.chars().nth(pos)))
        })
        .map(|(_, n)| n)
        .sum()
}

fn get_number_map(v: &str) -> HashMap<usize, usize> {
    let mut offset = 0;
    let numbers = v.split(|c: char| !c.is_ascii_digit()).enumerate().fold(
        HashMap::new(),
        |mut acc, (i, c)| {
            if let Ok(number) = c.parse::<usize>() {
                acc.insert(i + offset, number);
                offset += c.len();
            };
            acc
        },
    );
    numbers
}

fn get_neighbor_pos(
    pos: usize,
    number_length: usize,
    line_length: usize,
    input: &String,
) -> Vec<usize> {
    let mut neighbors = Vec::new();
    if pos % line_length > 0 {
        neighbors.append(&mut vec![pos - 1]);
    }
    if (pos + number_length) % line_length < line_length - 1 {
        neighbors.append(&mut vec![pos + number_length]);
    }
    if pos > line_length {
        neighbors.append(
            &mut (pos - line_length - 1..=pos - line_length + number_length)
                .collect::<Vec<usize>>(),
        );
    }
    if pos < (input.len() - line_length) {
        neighbors.append(
            &mut (pos + line_length - 1..=pos + line_length + number_length)
                .collect::<Vec<usize>>(),
        );
    }
    neighbors
}

fn check_sign(c: Option<char>) -> bool {
    match c {
        Some(c) => !c.is_ascii_digit() && c != '.',
        None => false,
    }
}

pub fn solve2(input: &str) -> usize {
    let (line_length, v) = parse(input);
    let numbers = get_number_map(&v);
    let gears: HashMap<usize, Vec<usize>> =
        numbers
            .iter()
            .fold(HashMap::new(), |mut acc, (&pos, &number)| {
                let number_length = number.to_string().len();
                let check_positions = get_neighbor_pos(pos, number_length, line_length, &v);
                for p in check_positions {
                    if check_gear(v.chars().nth(p)) {
                        acc.entry(p)
                            .and_modify(|n| n.push(number))
                            .or_insert(vec![number]);
                    }
                }
                acc
            });
    gears
        .iter()
        .filter_map(|(_, v)| {
            if v.len() > 1 {
                Some(v.iter().product::<usize>())
            } else {
                None
            }
        })
        .sum()
}

fn check_gear(c: Option<char>) -> bool {
    match c {
        Some(c) => c == '*',
        None => false,
    }
}

fn parse(input: &str) -> (usize, String) {
    let length = input.lines().next().unwrap().len();
    (length, input.lines().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 4361);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 467835);
    }
}
