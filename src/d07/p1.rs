use std::ops::Index;

use crate::util::file_reader;

pub fn start() {
    let input = file_reader("d07");
    let mut hands = input
        .into_iter()
        .map(|s| {
            let v = s
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            Hand::from(v[0].as_str(), v[1].clone().parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    for hand in &hands {
        println!("{:?}", hand);
    }
    hands.sort_by(|a, b| a.cmp(b));
    println!("end sort");
    let out = hands
        .into_iter()
        .enumerate()
        .map(|(x, h)| (x + 1) * h.bid)
        .sum::<usize>();
    println!("{}", out);
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Hand {
    label: String,
    hand_type: Option<HandType>,
    bid: usize
}

impl Hand {
    fn from(s: &str, bid: usize) -> Self {
        Self {
            label: s.to_string(),
            hand_type: None,
            bid
        }.evaluate()
    }

    fn evaluate(self) -> Self {
        let card_ordering = [
            'A',
            'K',
            'Q',
            'J',
            'T',
            '9',
            '8',
            '7',
            '6',
            '5',
            '4',
            '3',
            '2'
        ];
        let mut counts: [isize; 13] = [0; 13];
        for letter in &card_ordering {
            for char in self.label.chars() {
                if *letter == char {
                    let pos = card_ordering.iter().position(|&r| r == *letter).unwrap();
                    counts[pos] += 1;
                }
            }
        }

        let hand_type: HandType;
        let mut highest = 0;
        let mut highest_ind = 9999;
        // five
        for i in 0..counts.len() {
            if counts[i] > highest {
                highest = counts[i];
                highest_ind = i;
            }
        }
        let cn = highest;
        if cn == 5 {
            hand_type = HandType::Five(card_ordering[highest_ind])
        } else if cn == 4 {
            hand_type = HandType::Four(card_ordering[highest_ind])
        } else if cn == 3 {
            if counts.iter().collect::<Vec<_>>().contains(&&2) {
                hand_type = HandType::Full(card_ordering[highest_ind])
            } else {
                hand_type = HandType::Three(card_ordering[highest_ind])
            }
        } else if cn == 2 {
            let mut two_exists = false;
            for i in 0..counts.len() {
                if i != highest_ind {
                    if counts[i] == 2 {
                        two_exists = true;
                    }
                }
            }
            if two_exists {
                hand_type = HandType::TwoPair(card_ordering[highest_ind])
            } else {
                hand_type = HandType::OnePair(card_ordering[highest_ind])
            }
        } else {
            hand_type = HandType::High(card_ordering[highest_ind])
        }

        Self {
            label: self.label,
            hand_type: Some(hand_type),
            bid: self.bid
        }
    }

    fn score(&self) -> usize {
        self.hand_type.as_ref().unwrap().score()
    }

    fn compare<'a>(a: &'a Self, b: &'a Self) -> &'a Self {
        let cards = [
            'A',
            'K',
            'Q',
            'J',
            'T',
            '9',
            '8',
            '7',
            '6',
            '5',
            '4',
            '3',
            '2'
        ].into_iter().collect::<Vec<_>>();
        let mut card_scores = [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 ];
        card_scores.reverse();
        if a.score() > b.score() {
            a
        } else if b.score() > a.score() {
            b
        } else {
            let mut winner = a;
            for i in 0..5 {
                let a_let = a.label.chars().collect::<Vec<_>>()[i];
                let b_let = b.label.chars().collect::<Vec<_>>()[i];
                let a_score = card_scores[cards.iter().position(|&r| r == a_let).unwrap()];
                let b_score = card_scores[cards.iter().position(|&r| r == b_let).unwrap()];
                if a_score > b_score {
                    winner = a;
                    break
                } else if b_score > a_score {
                    winner = b;
                    break
                }
            }
            winner
        }
    }
}

impl Eq for Hand {

}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let res = Hand::compare(self, other);
        if res == self {
            std::cmp::Ordering::Greater
        } else if res == other {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    Five(char),
    Four(char),
    Full(char),
    Three(char),
    TwoPair(char),
    OnePair(char),
    High(char)
}

impl HandType {
    fn score(&self) -> usize {
        match self {
            HandType::Five(_) => 7,
            HandType::Four(_) => 6,
            HandType::Full(_) => 5,
            HandType::Three(_) => 4,
            HandType::TwoPair(_) => 3,
            HandType::OnePair(_) => 2,
            HandType::High(_) => 1,
        }
    }
}

impl Eq for HandType {

}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score() > other.score() {
            std::cmp::Ordering::Greater
        } else if self.score() < other.score() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}