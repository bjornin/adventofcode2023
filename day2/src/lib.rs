pub fn solve1(input: &str) -> usize {
    let v = parse(input);
    v.iter().filter_map(possible_game).sum()
}

fn possible_game(game: &String) -> Option<usize> {
    let mut game = game.split(": ");
    let game_nr: usize = game.next()?.split_whitespace().nth(1)?.parse().ok()?;
    let draws = game.next()?.split("; ");
    for d in draws {
        let cube = d.split(", ");
        for c in cube {
            let mut iter = c.split_whitespace();
            let count_str = iter.next()?;
            let count = count_str.parse::<usize>().ok().unwrap_or(0);
            match iter.next() {
                Some("red") => {
                    if count > 12 {
                        return None;
                    }
                }
                Some("green") => {
                    if count > 13 {
                        return None;
                    }
                }
                Some("blue") => {
                    if count > 14 {
                        return None;
                    }
                }
                _ => {}
            }
        }
    }
    Some(game_nr)
}

pub fn solve2(input: &str) -> usize {
    let v = parse(input);
    v.iter().filter_map(power_game).sum()
}

fn power_game(game: &String) -> Option<usize> {
    let mut game = game.split(": ");
    let _: usize = game.next()?.split_whitespace().nth(1)?.parse().ok()?;
    let draws = game.next()?.split("; ");
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for d in draws {
        let cube = d.split(", ");
        for c in cube {
            let mut iter = c.split_whitespace();
            let count_str = iter.next()?;
            let count = count_str.parse::<usize>().ok().unwrap_or(0);
            match iter.next() {
                Some("red") => {
                    if count > min_red {
                        min_red = count;
                    }
                }
                Some("green") => {
                    if count > min_green {
                        min_green = count;
                    }
                }
                Some("blue") => {
                    if count > min_blue {
                        min_blue = count;
                    }
                }
                _ => {}
            }
        }
    }
    let power = min_red * min_green * min_blue;
    Some(power)
}

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 8);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 2286);
    }
}
