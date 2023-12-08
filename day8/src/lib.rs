use std::collections::HashMap;

pub fn solve1(input: &str) -> usize {
    let (directions, nodes) = parse(input);
    let mut steps = 0;
    let (mut name, mut node) = nodes.get_key_value("AAA").unwrap();
    loop {
        for direction in directions.chars() {
            match direction {
                'L' => (name, node) = nodes.get_key_value(node.0).unwrap(),
                'R' => (name, node) = nodes.get_key_value(node.1).unwrap(),
                _ => panic!("Invalid direction"),
            }
            steps += 1;
            if *name == "ZZZ" {
                return steps;
            }
        }
    }
}

pub fn solve2(input: &str) -> usize {
    let (directions, nodes) = parse(input);
    let curr_nodes: Vec<&str> = nodes
        .iter()
        .filter_map(|(&k, _)| if k.ends_with('A') { Some(k) } else { None })
        .collect();
    println!("{:?}", curr_nodes);
    let periods: Vec<usize> = curr_nodes
        .iter()
        .map(|&n| {
            let mut steps = 0;
            let mut name = n;
            loop {
                for direction in directions.chars() {
                    match direction {
                        'L' => name = nodes.get(name).unwrap().0,
                        'R' => name = nodes.get(name).unwrap().1,
                        _ => panic!("Invalid direction"),
                    }
                    steps += 1;
                    if name.ends_with('Z') {
                        return steps;
                    }
                }
            }
        })
        .collect();
    // LCM of periods
    let mut ret = 1;
    for p in periods {
        ret = lcm(p, ret);
    }
    ret
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut map = HashMap::new();
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    lines.next();
    lines.for_each(|l| {
        let e: Vec<&str> = l
            .split(|c| c == '=' || c == '(' || c == ',' || c == ')')
            .filter_map(|s| {
                if s.trim().is_empty() {
                    None
                } else {
                    Some(s.trim())
                }
            })
            .collect();
        map.entry(e[0]).or_insert((e[1], e[2]));
    });
    (directions, map)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const INPUT2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn a1() {
        assert_eq!(solve1(INPUT1), 2);
        assert_eq!(solve1(INPUT2), 6);
    }

    const INPUT3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn a2() {
        let result = solve2(INPUT3);
        assert_eq!(result, 6);
    }
}
