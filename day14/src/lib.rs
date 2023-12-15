use std::collections::HashMap;

pub fn solve1(input: &str) -> usize {
    let mut grid = parse(input);
    tilt_north(&mut grid);
    calc(&grid)
}

pub fn solve2(input: &str) -> usize {
    let mut grid = parse(input);
    let mut cache = HashMap::new();
    let mut i = 1;
    loop {
        for _ in 0..4 {
            tilt_north(&mut grid);
            grid = rotate(&grid);
        }
        if let Some(first_i) = cache.insert(grid.clone(), i) {
            let period = i - first_i;
            if (1000000000 - first_i) % period == 0 {
                break;
            }
        }
        i += 1;
    }
    calc(&grid)
}

fn calc(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, r)| r.iter().filter(|&c| c == &'O').count() * (grid.len() - i))
        .sum()
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    let mut changed = true;
    while changed {
        changed = false;
        for r in 1..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r - 1][c] == '.' && grid[r][c] == 'O' {
                    grid[r - 1][c] = 'O';
                    grid[r][c] = '.';
                    changed = true;
                }
            }
        }
    }
}

fn rotate(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    (0..matrix[0].len())
        .map(|i| matrix.iter().rev().map(|row| row[i]).collect())
        .collect()
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 136);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 64);
    }
}
