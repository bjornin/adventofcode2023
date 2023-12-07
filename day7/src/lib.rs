mod a1;
mod a2;

pub fn solve1(input: &str) -> usize {
    let mut hands: Vec<a1::Hand> = input.trim().lines().map(a1::Hand::from).collect();
    hands.sort_by(a1::compare_set_and_high);
    hands.iter().enumerate().map(|(i, h)| h.bid * (i + 1)).sum()
}

pub fn solve2(input: &str) -> usize {
    let mut hands: Vec<a2::Hand> = input.trim().lines().map(a2::Hand::from).collect();
    hands.sort_by(a2::compare_set_and_high);
    hands.iter().enumerate().map(|(i, h)| h.bid * (i + 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    const INPUT1: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_compare_set_and_high() {
        let test_hands = [
            ("22222 0", "22222 0", Ordering::Equal),
            ("33332 0", "33333 0", Ordering::Less),
            ("33333 0", "33332 0", Ordering::Greater),
            ("22223 0", "33332 0", Ordering::Less),
            ("32222 0", "33332 0", Ordering::Less),
            ("23234 0", "23235 0", Ordering::Less),
            ("3AA22 0", "2KK44 0", Ordering::Greater),
            ("33232 0", "32332 0", Ordering::Greater),
            ("33232 0", "22334 0", Ordering::Greater),
            ("AA234 0", "234AA 0", Ordering::Greater),
            ("2345J 0", "JJ234 0", Ordering::Less),
        ];
        for hand in test_hands.iter() {
            let hand1 = a1::Hand::from(hand.0);
            let hand2 = a1::Hand::from(hand.1);
            assert_eq!(
                a1::compare_set_and_high(&hand1, &hand2),
                hand.2,
                "a: {:?} b:{:?}",
                hand1,
                hand2
            );
        }
    }

    #[test]
    fn a1() {
        let result = solve1(INPUT1);
        assert_eq!(result, 6440);
    }

    #[test]
    fn a2() {
        let result = solve2(INPUT1);
        assert_eq!(result, 5905);
    }
}
