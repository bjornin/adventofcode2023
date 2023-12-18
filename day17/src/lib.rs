use std::cmp::Reverse;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Down,
        }
    }
}

fn step(pos: (usize, usize), dir: Dir, steps: isize) -> (usize, usize) {
    let diff = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let (r, c) = pos;
    let (dr, dc) = diff[dir as usize];
    (
        (r as isize + dr * steps) as usize,
        (c as isize + dc * steps) as usize,
    )
}

pub fn solve1(input: &str) -> usize {
    let v = parse(input);
    heat(&v, 1, 3)
}

fn heat(v: &[Vec<usize>], minstep: isize, maxstep: isize) -> usize {
    let mut visited = HashMap::new();
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, (0, 0, None))));
    while let Some(node) = q.pop() {
        let (h, (r, c, d)) = node.0;
        if (r, c) == (v.len() - 1, v[0].len() - 1) {
            return h;
        }
        if let Some(prev) = visited.get(&(r, c, d.unwrap_or(Dir::Right))) {
            if h > *prev {
                continue;
            }
        }
        for nd in [Dir::Right, Dir::Down, Dir::Left, Dir::Up] {
            if let Some(dir) = d {
                if dir == nd.opposite() || dir == nd {
                    continue;
                }
            }
            let mut nh = h;
            for i in 1..=maxstep {
                let (nr, nc) = step((r, c), nd, i);
                if nr >= v.len() || nc >= v[0].len() {
                    continue;
                }
                nh += v[nr][nc];
                if i < minstep {
                    continue;
                }
                let k = (nr, nc, Some(nd));
                let next = Reverse((nh, k));
                match visited.entry((nr, nc, nd)) {
                    Occupied(mut e) => {
                        if nh < *e.get() {
                            e.insert(nh);
                            q.push(next);
                        }
                    }
                    Vacant(e) => {
                        e.insert(nh);
                        q.push(next);
                    }
                }
            }
        }
    }
    0
}

pub fn solve2(input: &str) -> usize {
    let v = parse(input);
    heat(&v, 4, 10)
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c: char| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r"
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533
    ";

    const INPUT2: &str = r"
    241343
    321545
    ";

    const INPUT3: &str = r"
    111111111111
    999999999991
    999999999991
    999999999991
    999999999991
    ";

    #[test]
    fn a1() {
        assert_eq!(solve1(INPUT2), 20);
        assert_eq!(solve1(INPUT1), 102);
    }

    #[test]
    fn a2() {
        assert_eq!(solve2(INPUT1), 94);
        assert_eq!(solve2(INPUT3), 71);
    }
}
