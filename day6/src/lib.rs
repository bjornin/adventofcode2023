pub fn solve1(input: &str) -> usize {
    let (time, dist) = parse(input);
    time.iter()
        .zip(dist.iter())
        .map(|(&a, &b)| Race::new(a, b).get_wins())
        .product()
}

pub fn solve2(input: &str) -> usize {
    let (time, dist) = parse(input);
    let t = time
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();
    let d = dist
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();
    Race::new(t, d).get_wins()
}

#[derive(Debug)]
struct Race {
    time: i64,
    dist: i64,
}

impl Race {
    fn new(time: i64, dist: i64) -> Self {
        Self { time, dist }
    }

    fn get_wins(self) -> usize {
        // ax^2 - bx + c = 0 (a=1, b=time, c=dist)
        let d = (-self.time * -self.time) - (4 * self.dist);
        let r1;
        let r2;
        match d {
            0 => {
                r1 = self.time as f64 / 2.0;
                r2 = r1;
            }
            d if d > 0 => {
                r1 = (self.time as f64 + (d as f64).sqrt()) / 2.0;
                r2 = (self.time as f64 - (d as f64).sqrt()) / 2.0;
            }
            _ => panic!("imaginary roots"),
        }
        r1.ceil() as usize - r2.floor() as usize - 1
    }
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut t = input.trim().lines();
    let time = t
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n: &str| n.parse().unwrap())
        .collect();
    let dist = t
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n: &str| n.parse().unwrap())
        .collect();
    (time, dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 288);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 71503);
    }
}
