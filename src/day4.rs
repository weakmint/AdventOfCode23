#[derive(Clone, Debug)]
struct Card {
    num: u32,
    scratched: Vec<u32>,
    winning: Vec<u32>,
}

fn get_cards(data: Vec<String>) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];
    for card in data {
        let card_parts: Vec<&str> = card
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let card_num = card_parts[1].replace(":", "");
        let numbers = &card_parts[2..];
        let pipe = numbers.into_iter().position(|x| x == &"|").unwrap();
        let scratched_str: Vec<&str> = numbers[0..pipe].to_vec();
        let winning_str: Vec<&str> = numbers[pipe + 1..].to_vec();

        cards.push(Card {
            num: card_num.parse().unwrap(),
            scratched: scratched_str.iter().map(|x| x.parse().unwrap()).collect(),
            winning: winning_str.iter().map(|x| x.parse().unwrap()).collect(),
        })
    }

    return cards;
}

fn get_matches(card: Card) -> u32 {
    return card
        .scratched
        .into_iter()
        .filter(|x| card.winning.contains(x))
        .count() as u32;
}

fn score_card(matches_counter: u32) -> u32 {
    match matches_counter {
        0 => 0,
        _ => 2u32.pow(matches_counter - 1),
    }
}

fn get_card_count(card: &Card, cards: &Vec<Card>) -> Vec<u32> {
    let copies = get_matches(card.clone());

    match copies {
        0 => return vec![],
        _ => {
            // 1, 4
            // 2 - 5
            let mut acc: Vec<u32> = (card.num + 1..card.num + 1 + copies).collect();
            //println!("{:#?} {:#?}", new_acc, acc);
            for i in acc.clone() {
                let copy = cards
                    .clone()
                    .into_iter()
                    .filter(|x| x.num == i)
                    .last()
                    .unwrap();

                acc.append(&mut get_card_count(&copy, &cards));
            }
            return acc.to_vec();
        }
    }
}

pub fn part1(data: Vec<String>) -> String {
    let cards = get_cards(data);
    let scores: Vec<u32> = cards
        .into_iter()
        .map(|x| score_card(get_matches(x)))
        .collect();
    return scores.iter().sum::<u32>().to_string();
}

pub fn part2(data: Vec<String>) -> String {
    let cards = get_cards(data);
    let mut all_cards = vec![];
    for card in &cards {
        all_cards.push(card.num);
        all_cards.append(&mut get_card_count(card, &cards));
        all_cards.sort();
        println!("{} {:#?}", card.num, all_cards);
    }

    return all_cards.len().to_string();
}
