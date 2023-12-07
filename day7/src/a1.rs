use std::{cmp::Ordering, collections::HashMap};

pub fn compare_set_and_high(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    let mut a_score: HashMap<&Card, usize> = HashMap::new();
    let mut b_score: HashMap<&Card, usize> = HashMap::new();
    a.cards.iter().for_each(|c| {
        a_score.entry(c).and_modify(|c| *c += 1).or_insert(1);
    });
    b.cards.iter().for_each(|c| {
        b_score.entry(c).and_modify(|c| *c += 1).or_insert(1);
    });

    let a_score_sum = a_score.iter().fold(0, |acc, (_, &v)| acc + v.pow(2));
    let b_score_sum = b_score.iter().fold(0, |acc, (_, &v)| acc + v.pow(2));
    match a_score_sum.cmp(&b_score_sum) {
        Ordering::Equal => a.cmp(b),
        o => o,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(s: char) -> Card {
        match s {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid: usize,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: usize) -> Self {
        Self { cards, bid }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let mut line = s.split_ascii_whitespace();
        let hand = line.next().unwrap().chars().map(Card::from).collect();
        let bid = line.next().unwrap().parse().unwrap();
        Self::new(hand, bid)
    }
}
