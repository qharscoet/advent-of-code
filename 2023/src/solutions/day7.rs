use std::cmp::Ordering;

use crate::solution::Solution;

pub struct Day7;

#[derive(Debug, Clone, Eq)]
pub struct Hand {
    cards: Vec<u32>,
    hand_type: HandType,
    bid: u32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if std::mem::discriminant(&self.hand_type) == std::mem::discriminant(&other.hand_type) {
            let val = std::iter::zip(self.cards.clone(), other.cards.clone())
                .skip_while(|(a, b)| a == b)
                .next()
                .unwrap();
            val.0.cmp(&val.1)
        } else {
            (self.hand_type as u32).cmp(&(other.hand_type as u32))
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

fn get_hand_type(cards: &Vec<u32>) -> HandType {
    use HandType::*;

    let mut counts = [0u32; 13];
    for c in cards.iter() {
        /* -1 because values goes from 2 to 14 */
        counts[*c as usize - 2] += 1;
    }

    let mut groups = [0u32; 6];

    for count in counts {
        groups[count as usize] += 1;
    }

    return if groups[5] > 0 {
        FiveOfAKind
    } else if groups[4] > 0 {
        FourOfAKind
    } else if groups[3] == 1 && groups[2] == 1 {
        FullHouse
    } else if groups[3] == 1 && groups[1] == 2 {
        ThreeOfAKind
    } else if groups[2] == 2 {
        TwoPair
    } else if groups[2] == 1 {
        OnePair
    } else {
        HighCard
    };
}

impl std::str::FromStr for Hand {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((cards_str, bid_str)) = s.split_once(' ') else {
            return Err("String not in the right format !");
        };

        let bid: u32 = bid_str.parse().unwrap();

        let cards_result: Result<Vec<_>, _> = cards_str
            .chars()
            .map(|c| match c {
                '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    c.to_digit(10).ok_or("invalid number")
                }
                'T' => Ok(10u32),
                'J' => Ok(11u32),
                'Q' => Ok(12u32),
                'K' => Ok(13u32),
                'A' => Ok(14u32),
                _ => return Err("Cards not valid"),
            })
            .collect();

        let cards = cards_result.unwrap();
        let hand_type = get_hand_type(&cards);

        Ok(Hand {
            cards: cards,
            bid: bid,
            hand_type: hand_type,
        })
    }
}

impl Solution for Day7 {
    type Input = Vec<Hand>;
    type ReturnType = u32;
    const DAY: u32 = 7;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        lines.map(|s| s.parse().unwrap()).collect()
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let mut copy = input.clone();
        copy.sort();
        copy.iter()
            .enumerate()
            .map(|(idx, hand)| (idx as u32 + 1) * hand.bid)
            .sum()
    }
    fn second_part(&self, _input: &Self::Input) -> Self::ReturnType {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Day7;
    use crate::solution::Solution;

    static INPUT_TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    static INPUT_TEST_2: &str = "";

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day7.parse_input(lines);
        assert_eq!(Day7.first_part(&input), 6440)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST_2.split('\n').map(|s| s.to_string());
        let input = Day7.parse_input(lines);
        assert_eq!(Day7.second_part(&input), u32::MAX)
    }
}
