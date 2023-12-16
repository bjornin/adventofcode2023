use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve1(input: &str) -> usize {
    let v: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();
    energy(&v, (0, 0, Dir::Right))
}

pub fn solve2(input: &str) -> usize {
    let v: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();

    let i1 = (0..v.len()).map(|r| energy(&v, (0, r, Dir::Right)));
    let i2 = (0..v.len()).map(|r| energy(&v, (v[0].len() - 1, r, Dir::Left)));
    let i3 = (0..v[0].len()).map(|c| energy(&v, (c, 0, Dir::Down)));
    let i4 = (0..v.len()).map(|c| energy(&v, (c, v.len() - 1, Dir::Up)));
    i1.chain(i2).chain(i3).chain(i4).max().unwrap()
}

fn energy(v: &Vec<Vec<char>>, start: (usize, usize, Dir)) -> usize {
    let mut visited: HashMap<(usize, usize, Dir), usize> = HashMap::new();
    let mut q = vec![start];
    while let Some(next) = q.pop() {
        match visited.entry(next) {
            Occupied(_) => {
                continue;
            }
            Vacant(e) => {
                e.insert(1);
            }
        }
        let (c, r, dir) = next;
        match v.get(r).and_then(|l| l.get(c)) {
            Some('.') => match dir {
                Dir::Up => {
                    if r > 0 {
                        q.push((c, r - 1, Dir::Up));
                    }
                }
                Dir::Down => {
                    if r < v.len() - 1 {
                        q.push((c, r + 1, Dir::Down));
                    }
                }
                Dir::Left => {
                    if c > 0 {
                        q.push((c - 1, r, Dir::Left));
                    }
                }
                Dir::Right => {
                    if c < v[0].len() - 1 {
                        q.push((c + 1, r, Dir::Right));
                    }
                }
            },
            Some('-') => match dir {
                Dir::Up | Dir::Down => {
                    if c > 0 {
                        q.push((c - 1, r, Dir::Left));
                    }
                    if c < v[0].len() - 1 {
                        q.push((c + 1, r, Dir::Right));
                    }
                }
                Dir::Left => {
                    if c > 0 {
                        q.push((c - 1, r, Dir::Left));
                    }
                }
                Dir::Right => {
                    if c < v[0].len() - 1 {
                        q.push((c + 1, r, Dir::Right));
                    }
                }
            },
            Some('\\') => match dir {
                Dir::Up => {
                    if c > 0 {
                        q.push((c - 1, r, Dir::Left));
                    }
                }
                Dir::Down => {
                    if c < v[0].len() - 1 {
                        q.push((c + 1, r, Dir::Right));
                    }
                }
                Dir::Left => {
                    if r > 0 {
                        q.push((c, r - 1, Dir::Up));
                    }
                }
                Dir::Right => {
                    if r < v.len() - 1 {
                        q.push((c, r + 1, Dir::Down));
                    }
                }
            },
            Some('/') => match dir {
                Dir::Up => {
                    if c < v[0].len() - 1 {
                        q.push((c + 1, r, Dir::Right));
                    }
                }
                Dir::Down => {
                    if c > 0 {
                        q.push((c - 1, r, Dir::Left));
                    }
                }
                Dir::Left => {
                    if r < v.len() - 1 {
                        q.push((c, r + 1, Dir::Down));
                    }
                }
                Dir::Right => {
                    if r > 0 {
                        q.push((c, r - 1, Dir::Up));
                    }
                }
            },
            Some('|') => match dir {
                Dir::Up => {
                    if r > 0 {
                        q.push((c, r - 1, Dir::Up));
                    }
                }
                Dir::Down => {
                    if r < v.len() - 1 {
                        q.push((c, r + 1, Dir::Down));
                    }
                }
                Dir::Left | Dir::Right => {
                    if r > 0 {
                        q.push((c, r - 1, Dir::Up));
                    }
                    if r < v.len() - 1 {
                        q.push((c, r + 1, Dir::Down));
                    }
                }
            },
            Some(_) => panic!("Unknown mirror"),
            None => panic!("Out of bounds"),
        }
    }
    visited
        .iter()
        .fold(HashSet::new(), |mut acc, ((c, r, _), _)| {
            acc.insert((*c, *r));
            acc
        })
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 46);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 51);
    }
}
