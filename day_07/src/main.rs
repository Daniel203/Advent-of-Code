use std::{collections::HashMap, fs, num::ParseIntError, ops::Index, str::FromStr};

static INPUT_DATA: &str = "src/inputs/input.txt";
type InputType = Vec<Hand>;

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    score: usize,
}

impl FromStr for Hand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();

        return Ok(Hand {
            cards: data[0].chars().collect(),
            bid: data[1].parse::<usize>().unwrap(),
            score: 0,
        });
    }
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            cards: vec![],
            bid: 0,
            score: 0,
        }
    }
}

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const CARDS_2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn main() {
    let res1 = part_one(INPUT_DATA);
    let res2 = part_two(INPUT_DATA);

    println!("1: {}", res1);
    println!("2: {}", res2);
}

fn parse_input(path: &str) -> InputType {
    let input = fs::read_to_string(path).expect("Error while reading the file!");
    let parsed_input = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| Hand::from_str(x).unwrap_or(Hand::default()))
        .collect();
    return parsed_input;
}

fn get_hand_score(hand: &Hand) -> usize {
    let mut cards: HashMap<char, usize> = HashMap::new();

    hand.cards.iter().for_each(|&x| {
        let card_score = cards.get(&x).unwrap_or(&0);
        cards.insert(x, card_score + 1);
    });

    return cards.values().map(|&x| x.pow(2)).sum();
}

fn get_hand_score_2(hand: &Hand) -> usize {
    let mut cards: HashMap<char, usize> = HashMap::new();

    hand.cards.iter().for_each(|&x| {
        let card_score = cards.get(&x).unwrap_or(&0);
        cards.insert(x, card_score + 1);
    });

    if hand.cards.contains(&'J') && cards.get(&'J').unwrap() != &5 {
        let n_j = &cards.get(&'J').copied().unwrap_or(0);
        cards.remove(&'J');
        let biggest_card = cards.iter().max_by_key(|&(_, &v)| v).unwrap().0;
        cards
            .entry(*biggest_card)
            .and_modify(|v| *v += *n_j)
            .or_insert(0);
    }

    return cards.values().map(|&x| x.pow(2)).sum();
}

fn part_one(path: &str) -> usize {
    let data = parse_input(path);

    let mut data_scored = data
        .iter()
        .map(|x| Hand {
            cards: x.cards.clone(),
            bid: x.bid,
            score: get_hand_score(x),
        })
        .collect::<Vec<Hand>>();

    data_scored.sort_by_key(|x| x.score);
    data_scored.sort_by(|a, b| {
        let ordering = b.score.cmp(&a.score);

        if ordering == std::cmp::Ordering::Equal {
            let mut i = 0;
            while i < a.cards.len() {
                let index_a = CARDS.iter().position(|&x| x == a.cards[i]).unwrap();
                let index_b = CARDS.iter().position(|&x| x == b.cards[i]).unwrap();

                if index_a != index_b {
                    return index_b.cmp(&index_a);
                }

                i += 1;
            }
        }

        return std::cmp::Ordering::Equal;
    });

    data_scored.iter().for_each(|x| println!("{:?}", x));
    data_scored
        .iter()
        .enumerate()
        .map(|(i, x)| x.bid * (i + 1))
        .sum()
}

fn part_two(path: &str) -> usize {
    let data = parse_input(path);

    let mut data_scored = data
        .iter()
        .map(|x| {
            let score = get_hand_score_2(x);
            Hand {
                cards: x.cards.clone(),
                bid: x.bid,
                score: score,
            }
        })
        .collect::<Vec<Hand>>();

    data_scored.sort_by_key(|x| x.score);
    data_scored.sort_by(|a, b| {
        let ordering = b.score.cmp(&a.score);

        if ordering == std::cmp::Ordering::Equal {
            let mut i = 0;
            while i < a.cards.len() {
                let index_a = CARDS_2.iter().position(|&x| x == a.cards[i]).unwrap();
                let index_b = CARDS_2.iter().position(|&x| x == b.cards[i]).unwrap();

                if index_a != index_b {
                    return index_b.cmp(&index_a);
                }

                i += 1;
            }
        }

        return std::cmp::Ordering::Equal;
    });

    // data_scored.iter().filter(|x| x.score >= 6).for_each(|x| println!("{:?}", x));

    data_scored
        .iter()
        .enumerate()
        .map(|(i, x)| x.bid * (i + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST_DATA: &str = "src/inputs/test_input.txt";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_TEST_DATA), 6440);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TEST_DATA), 5905);
    }
}
