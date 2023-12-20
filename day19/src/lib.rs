use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Part {
    categories: HashMap<char, usize>,
}

impl Part {
    fn rating(&self) -> usize {
        self.categories.values().sum()
    }

    fn accept(&self, rules: &HashMap<&str, Vec<Condition>>) -> bool {
        let mut next = "in";
        while next != "A" && next != "R" {
            let rule = rules.get(next).unwrap();
            for c in rule {
                if let Some(cat) = c.category {
                    let Some(num) = c.num else {
                        continue;
                    };
                    let Some(ordering) = c.ordering else {
                        continue;
                    };
                    if self.categories[&cat].cmp(&num) == ordering {
                        next = &c.next;
                        break;
                    }
                } else {
                    next = &c.next;
                    break;
                }
            }
        }
        next == "A"
    }
}

impl From<&str> for Part {
    fn from(input: &str) -> Self {
        let mut categories = HashMap::new();
        for p in input.trim_matches(|c| c == '{' || c == '}').split(',') {
            let mut kv = p.split('=');
            let k = kv.next().unwrap().chars().next().unwrap();
            let v = kv.next().unwrap().parse().unwrap();
            categories.insert(k, v);
        }
        Self { categories }
    }
}

#[derive(Debug)]
struct Condition {
    category: Option<char>,
    num: Option<usize>,
    ordering: Option<Ordering>,
    next: String,
}

impl From<&str> for Condition {
    fn from(input: &str) -> Self {
        if !input.contains(':') {
            Self {
                category: None,
                num: None,
                ordering: None,
                next: input.to_string(),
            }
        } else {
            let re = Regex::new(r"([x|m|a|s])([<|>])(\d+):(\w+)").unwrap();
            if let Some(caps) = re.captures(input) {
                let c = caps.get(1).unwrap().as_str().chars().next().unwrap();
                let o = caps.get(2).unwrap().as_str();
                let d = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let n = caps.get(4).unwrap().as_str();
                let ordering = match o {
                    "<" => Some(Ordering::Less),
                    ">" => Some(Ordering::Greater),
                    _ => None,
                };
                Self {
                    category: Some(c),
                    num: Some(d),
                    ordering,
                    next: n.to_string(),
                }
            } else {
                panic!("Invalid condition");
            }
        }
    }
}

pub fn solve1(input: &str) -> usize {
    let (r, p) = parse(input);
    p.iter().filter(|p| p.accept(&r)).map(|p| p.rating()).sum()
}

pub fn solve2(input: &str) -> usize {
    let (r, _) = parse(input);
    let mut combo = HashMap::new();
    combo.insert('x', (1, 4000));
    combo.insert('m', (1, 4000));
    combo.insert('s', (1, 4000));
    combo.insert('a', (1, 4000));
    accepted_combo("in", &r, &mut combo)
}

fn accepted_combo(
    rule: &str,
    rules: &HashMap<&str, Vec<Condition>>,
    combo: &mut HashMap<char, (usize, usize)>,
) -> usize {
    if rule == "A" {
        return combo.values().map(|(min, max)| max - min + 1).product();
    } else if rule == "R" {
        return 0;
    }
    let conditions = rules.get(rule).unwrap();
    let mut total = 0;
    let mut rest_combo = combo.clone();
    for c in conditions {
        if let Some(cat) = c.category {
            let Some(num) = c.num else {
                continue;
            };
            let Some(ordering) = c.ordering else {
                continue;
            };
            let mut new_combo = rest_combo.clone();
            let l = rest_combo[&cat];
            match ordering {
                Ordering::Less => {
                    new_combo.insert(cat, (l.0, num - 1));
                    rest_combo.insert(cat, (num, l.1));
                }
                Ordering::Greater => {
                    new_combo.insert(cat, (num + 1, l.1));
                    rest_combo.insert(cat, (l.0, num));
                }
                _ => panic!("Invalid ordering"),
            };
            total += accepted_combo(&c.next, rules, &mut new_combo);
        } else {
            total += accepted_combo(&c.next, rules, &mut rest_combo);
        }
    }
    total
}

fn parse(input: &str) -> (HashMap<&str, Vec<Condition>>, Vec<Part>) {
    let s = input.split_once("\n\n").unwrap();
    let rules = s.0.trim().lines().fold(HashMap::new(), |mut acc, r| {
        let mut sp = r.trim().split(|c| c == '{' || c == '}');
        let name = sp.next().unwrap();
        let conds: Vec<Condition> = sp.next().unwrap().split(',').map(Condition::from).collect();
        acc.insert(name, conds);
        acc
    });
    let parts = s.1.lines().map(Part::from).collect();
    (rules, parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 19114);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 167409079868000);
    }
}
