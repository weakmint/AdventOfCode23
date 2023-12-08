use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    cards: String,
    bid: i32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let a_hand_type = get_type(self.cards.clone());
        let b_hand_type = get_type(other.cards.clone());
        let mut ord = a_hand_type.cmp(&b_hand_type);

        if ord == Ordering::Equal {
            for (i, card) in self.cards.chars().into_iter().enumerate() {
                ord = compare_card(card, other.cards.chars().collect::<Vec<_>>()[i]);
                if ord != Ordering::Equal {
                    return ord;
                }
            }
        }
        return ord;
    }
}

fn compare_card(a: char, b: char) -> Ordering {
    let map = HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', 11), ('T', 10)]);
    let a_int = if a.is_digit(10) {
        a.to_digit(10).unwrap()
    } else {
        map[&a]
    };
    let b_int = if b.is_digit(10) {
        b.to_digit(10).unwrap()
    } else {
        map[&b]
    };

    return a_int.cmp(&b_int);
}

fn get_type(hand: String) -> i32 {
    let mut card_counts: HashMap<char, i32> = HashMap::new();
    for c in hand.chars() {
        let mut value = 1;
        if card_counts.contains_key(&c) {
            value += card_counts[&c];
        }
        card_counts.insert(c, value);
    }

    if card_counts.contains_key(&'1') {
        let mut max: char = 'L';
        for (c, count) in card_counts.clone() {
            if c != '1' {
                if max == 'L' || card_counts[&max] < count {
                    max = c;
                }
            }
        }

        if max != 'L' {
            card_counts.insert(max, card_counts[&'1'] + card_counts[&max]);
            card_counts.remove(&'1');
        }
    }

    if card_counts.len() == 1 {
        // 5 of a kind
        return 6;
    }

    if card_counts.len() == 2 {
        if card_counts.values().any(|x| x == &1 || x == &4) {
            // 4 of a kind
            return 5;
        } else {
            // full house
            return 4;
        }
    }

    if card_counts.len() == 3 {
        if card_counts.values().any(|x| x == &3) {
            // three of a kind
            return 3;
        } else {
            // two pair
            return 2;
        }
    }

    if card_counts.len() == 4 {
        // one pair
        return 1;
    }

    // high card
    return 0;
}

fn get_hands(data: Vec<String>) -> Vec<Hand> {
    return data
        .into_iter()
        .map(|x| Hand {
            cards: x.split(" ").collect::<Vec<_>>()[0].to_owned(),
            bid: x.split(" ").collect::<Vec<_>>()[1].parse().unwrap(),
        })
        .collect::<Vec<_>>();
}

fn get_hands2(data: Vec<String>) -> Vec<Hand> {
    return data
        .into_iter()
        .map(|x| Hand {
            cards: x
                .split(" ")
                .map(|x| x.replace("J", "1"))
                .collect::<Vec<_>>()[0]
                .to_owned(),
            bid: x.split(" ").collect::<Vec<_>>()[1].parse().unwrap(),
        })
        .collect::<Vec<_>>();
}

pub fn part1(data: Vec<String>) -> String {
    let mut sum: i32 = 0;
    let mut hands = get_hands(data);
    hands.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    for (i, hand) in hands.clone().into_iter().enumerate() {
        sum += (i as i32 + 1) * hand.bid;
    }
    return sum.to_string();
}

pub fn part2(data: Vec<String>) -> String {
    let mut sum: i32 = 0;
    let mut hands = get_hands2(data);
    hands.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    for (i, hand) in hands.clone().into_iter().enumerate() {
        sum += (i as i32 + 1) * hand.bid;
    }
    return sum.to_string();
}
