use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    North,
    East,
    South,
    West,
}

pub fn solve1(input: &str) -> usize {
    let (_, _, map) = parse(input);
    let start = map
        .iter()
        .find(|(_, &c)| c == 'S')
        .map(|(coord, _)| *coord)
        .unwrap();
    let pipe = get_pipe(&map, start);
    pipe.len() / 2
}

pub fn solve2(input: &str) -> usize {
    let (max_row, max_col, mut map) = parse(input);
    let start = map
        .iter()
        .find(|(_, &c)| c == 'S')
        .map(|(coord, _)| *coord)
        .unwrap();
    let mut pipe = get_pipe(&map, start);
    update_start(&mut map, start);
    update_pipe(&map, &mut pipe, max_row, max_col);
    map.iter()
        .filter(|(k, _)| !pipe.contains_key(k))
        .map(|(&k, _)| is_inside(&pipe, max_row, max_col, k))
        .filter(|&b| b)
        .count()
}

fn update_pipe(
    map: &HashMap<(usize, usize), char>,
    pipe: &mut HashMap<(usize, usize), i8>,
    max_row: usize,
    max_col: usize,
) {
    let mut q = vec![];
    for row in 0..max_row {
        for col in 0..max_col {
            let coord = (row, col);
            if let Entry::Occupied(mut p) = pipe.entry(coord) {
                match map.get(&coord) {
                    Some('|') => {
                        p.insert(1);
                    }
                    Some('-') => {
                        continue;
                    }
                    Some('L') => {
                        q.push(Dir::North);
                    }
                    Some('F') => {
                        q.push(Dir::South);
                    }
                    Some('7') => {
                        if let Some(dir) = q.get(0) {
                            match dir {
                                Dir::North => {
                                    p.insert(1);
                                }
                                Dir::South => {
                                    p.insert(0);
                                }
                                _ => panic!("Invalid dir"),
                            }
                        }
                        q.clear();
                    }
                    Some('J') => {
                        if let Some(dir) = q.get(0) {
                            match dir {
                                Dir::North => {
                                    p.insert(0);
                                }
                                Dir::South => {
                                    p.insert(1);
                                }
                                _ => panic!("Invalid dir"),
                            }
                        }
                        q.clear();
                    }
                    _ => continue,
                }
            }
        }
    }
}

fn update_start(map: &mut HashMap<(usize, usize), char>, start: (usize, usize)) {
    let mut connections = vec![];
    for dir in &[Dir::North, Dir::East, Dir::West, Dir::South] {
        if start.0 == 0 && dir == &Dir::North {
            continue;
        }
        let next = match dir {
            Dir::East => (start.0, start.1 + 1),
            Dir::West => (start.0, start.1 - 1),
            Dir::North => (start.0 - 1, start.1),
            Dir::South => (start.0 + 1, start.1),
        };
        match map.get(&next) {
            Some('F') => {
                if dir == &Dir::North || dir == &Dir::West {
                    connections.push(dir);
                }
            }
            Some('J') => {
                if dir == &Dir::South || dir == &Dir::East {
                    connections.push(dir);
                }
            }
            Some('7') => {
                if dir == &Dir::North || dir == &Dir::East {
                    connections.push(dir);
                }
            }
            Some('L') => {
                if dir == &Dir::South || dir == &Dir::West {
                    connections.push(dir);
                }
            }
            Some('|') => {
                if dir == &Dir::South || dir == &Dir::North {
                    connections.push(dir);
                }
            }
            Some('-') => {
                if dir == &Dir::West || dir == &Dir::East {
                    connections.push(dir);
                }
            }
            _ => continue,
        }
    }
    connections.sort();
    let c = match connections.as_slice() {
        [Dir::North, Dir::South] => '|',
        [Dir::East, Dir::West] => '-',
        [Dir::North, Dir::West] => 'J',
        [Dir::South, Dir::West] => '7',
        [Dir::North, Dir::East] => 'L',
        [Dir::East, Dir::South] => 'F',
        _ => panic!("Invalid start"),
    };
    map.insert(start, c);
}

fn is_inside(
    pipe: &HashMap<(usize, usize), i8>,
    max_row: usize,
    max_col: usize,
    coord: (usize, usize),
) -> bool {
    let (row, col) = coord;
    if row == 0 || row == max_row - 1 || col == 0 || col == max_col - 1 {
        return false;
    }
    let mut intersects = 0;
    for c in col..max_col {
        if let Some(p) = pipe.get(&(row, c)) {
            intersects += p;
        }
    }
    intersects % 2 == 1
}

fn get_pipe(
    map: &HashMap<(usize, usize), char>,
    start: (usize, usize),
) -> HashMap<(usize, usize), i8> {
    let mut pipe: HashMap<(usize, usize), i8> = HashMap::new();
    let mut next = start;
    loop {
        let point = 0;
        match map.get(&next) {
            Some('S') => {
                if !pipe.is_empty() {
                    // Update point for start
                    break;
                }
                next = get_next(
                    map,
                    next,
                    &pipe,
                    &[Dir::North, Dir::East, Dir::West, Dir::South],
                );
            }
            Some('|') => {
                // pipe.insert(next, 1);
                next = get_next(map, next, &pipe, &[Dir::North, Dir::South]);
            }
            Some('-') => {
                // pipe.insert(next, 0);
                next = get_next(map, next, &pipe, &[Dir::East, Dir::West]);
            }
            Some('J') => {
                // pipe.insert(next, 0);
                next = get_next(map, next, &pipe, &[Dir::North, Dir::West]);
            }
            Some('7') => {
                // pipe.insert(next, 0);
                next = get_next(map, next, &pipe, &[Dir::South, Dir::West]);
            }
            Some('L') => {
                // pipe.insert(next, 1);
                next = get_next(map, next, &pipe, &[Dir::North, Dir::East]);
            }
            Some('F') => {
                // pipe.insert(next, 1);
                next = get_next(map, next, &pipe, &[Dir::South, Dir::East]);
            }
            _ => panic!("Invalid char"),
        }
        pipe.insert(next, point);
    }
    pipe
}

fn get_next(
    map: &HashMap<(usize, usize), char>,
    coord: (usize, usize),
    visited: &HashMap<(usize, usize), i8>,
    dirs: &[Dir],
) -> (usize, usize) {
    for dir in dirs {
        if coord.0 == 0 && dir == &Dir::North {
            continue;
        }
        let next = match dir {
            Dir::East => (coord.0, coord.1 + 1),
            Dir::West => (coord.0, coord.1 - 1),
            Dir::North => (coord.0 - 1, coord.1),
            Dir::South => (coord.0 + 1, coord.1),
        };
        if let Some(c) = map.get(&next) {
            match c {
                'S' => {
                    if visited.len() > 2 {
                        return next;
                    }
                }
                '|' => {
                    if (dir == &Dir::North || dir == &Dir::South) && !visited.contains_key(&next) {
                        return next;
                    }
                }
                '-' => {
                    if (dir == &Dir::East || dir == &Dir::West) && !visited.contains_key(&next) {
                        return next;
                    }
                }
                'J' => {
                    if (dir == &Dir::South || dir == &Dir::East) && !visited.contains_key(&next) {
                        return next;
                    }
                }
                'L' => {
                    if (dir == &Dir::South || dir == &Dir::West) && !visited.contains_key(&next) {
                        return next;
                    }
                }
                'F' => {
                    if (dir == &Dir::North || dir == &Dir::West) && !visited.contains_key(&next) {
                        return next;
                    }
                }
                '7' => {
                    if (dir == &Dir::North || dir == &Dir::East) && !visited.contains_key(&next) {
                        return next;
                    }
                }
                _ => continue,
            }
        }
    }
    coord
}

fn parse(input: &str) -> (usize, usize, HashMap<(usize, usize), char>) {
    let mut map = HashMap::new();
    let max_row = input.lines().count();
    let max_col = input.lines().next().unwrap().trim().chars().count();
    input.lines().enumerate().for_each(|(row, line)| {
        line.trim().chars().enumerate().for_each(|(col, c)| {
            let coord = (row, col);
            map.insert(coord, c);
        });
    });
    (max_row, max_col, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....    
";
    const INPUT2: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[test]
    fn a1() {
        assert_eq!(solve1(INPUT1), 4);
        assert_eq!(solve1(INPUT2), 8);
    }

    const INPUT3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
    const INPUT4: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
    
";
    const INPUT5: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn a2() {
        assert_eq!(solve2(INPUT1), 1);
        assert_eq!(solve2(INPUT3), 4);
        assert_eq!(solve2(INPUT4), 8);
        assert_eq!(solve2(INPUT5), 10);
    }
}
