use std::vec;

#[derive(Debug, Default)]
struct Mapline {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl Mapline {
    fn new(data: &str) -> Self {
        let mut s = data.split_ascii_whitespace();
        let destination_start = s.next().unwrap().parse().unwrap();
        let source_start = s.next().unwrap().parse().unwrap();
        let length = s.next().unwrap().parse().unwrap();
        Self {
            destination_start,
            source_start,
            length,
        }
    }
}

#[derive(Debug)]
struct Mapper {
    maps: Vec<Mapline>,
}

impl Mapper {
    fn new() -> Self {
        Self { maps: vec![] }
    }

    fn add_map(&mut self, map: &str) {
        self.maps.push(Mapline::new(map));
    }

    pub fn check(&self, source: usize) -> usize {
        for map in &self.maps {
            if source >= map.source_start && source < map.source_start + map.length {
                let offset = source - map.source_start;
                return map.destination_start + offset;
            }
        }
        source
    }
}

pub fn solve1(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    find_min(seeds, &maps)
}

pub fn solve2(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    seeds
        .chunks(2)
        .map(|chunk| {
            if let [start, size] = chunk {
                let mut seeds = vec![];
                for i in 0..*size {
                    seeds.push(start + i);
                }
                println!("{} {}", start, size);
                find_min(seeds, &maps)
            } else {
                panic!("Invalid chunk");
            }
        })
        .min()
        .unwrap()
}

fn find_min(seed: Vec<usize>, maps: &Vec<Mapper>) -> usize {
    seed.iter()
        .map(|&s| {
            let mut source = s;
            for map in maps {
                source = map.check(source);
            }
            source
        })
        .min()
        .unwrap()
}

fn parse(input: &str) -> (Vec<usize>, Vec<Mapper>) {
    let mut split = input.trim().split("\n\n");
    let seeds = split
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut maps = vec![];
    split.for_each(|s| {
        let mut mapper = Mapper::new();
        s.split(":\n").nth(1).unwrap().split('\n').for_each(|l| {
            mapper.add_map(l);
        });
        maps.push(mapper);
    });

    (seeds, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 35);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 46);
    }
}
