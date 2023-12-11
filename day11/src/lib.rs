use std::collections::HashMap;

pub fn solve1(input: &str) -> usize {
    let (_, _, universe) = parse(input, 1);
    let dists = get_dists(universe);
    dists.iter().sum()
}

pub fn solve2(input: &str) -> usize {
    let (_, _, universe) = parse(input, 999999);
    let dists = get_dists(universe);
    dists.iter().sum()
}

fn get_dists(universe: HashMap<usize, (isize, isize)>) -> Vec<usize> {
    let universe_size = universe.len();
    let dists: Vec<usize> = (0..universe_size)
        .flat_map(|i| {
            (0..universe_size)
                .filter_map(|j| {
                    if j > i {
                        let (x1, y1) = universe.get(&i).unwrap();
                        let (x2, y2) = universe.get(&j).unwrap();
                        Some(((x1 - x2).abs() + (y1 - y2).abs()) as usize)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect();
    dists
}

fn parse(input: &str, multiple: usize) -> (usize, usize, HashMap<usize, (isize, isize)>) {
    let mut result: HashMap<usize, (isize, isize)> = HashMap::new();
    let rows = input.lines().count();
    let expand_rows: Vec<usize> = (0..rows)
        .filter(|row| input.lines().nth(*row).unwrap().chars().all(|c| c == '.'))
        .collect();
    let cols = input.lines().next().unwrap().len();
    let expand_cols: Vec<usize> = (0..cols)
        .filter(|col| input.lines().all(|l| l.chars().nth(*col).unwrap() == '.'))
        .collect();
    let mut galaxy = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                let r = expand_rows
                    .iter()
                    .fold(0, |acc, e| if *e < row { acc + 1 } else { acc })
                    * multiple;
                let c = expand_cols
                    .iter()
                    .fold(0, |acc, e| if *e < col { acc + 1 } else { acc })
                    * multiple;
                result.insert(galaxy, ((col + c) as isize, (row + r) as isize));
                galaxy += 1;
            }
        }
    }
    let max_cols = expand_cols.len() + cols;
    let max_rows = expand_rows.len() + rows;
    (max_cols, max_rows, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 374);
    }
}
