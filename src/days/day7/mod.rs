use std::cmp::Ordering;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::common::input_reader;

mod tests;
pub struct Day;


pub struct Hand {
    //cards: Vec<i32>,
    card1: i32,
    card2: i32,
    card3: i32,
    card4: i32,
    card5: i32,
    bid: i32
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

lazy_static! {
    static ref CARDS: HashMap<char, i32> = {
        let mut map = HashMap::new();
        map.insert('2', 2);
        map.insert('3', 3);
        map.insert('4', 4);
        map.insert('5', 5);
        map.insert('6', 6);
        map.insert('7', 7);
        map.insert('8', 8);
        map.insert('9', 9);
        map.insert('T', 10);
        map.insert('J', 11);
        map.insert('Q', 12);
        map.insert('K', 13);
        map.insert('A', 14);
        map
    };
}
impl Hand {
    pub fn new(data: String) -> Self {
        let parts = data.split(' ').collect::<Vec<&str>>();
        let bid = parts[1].trim().parse::<i32>().unwrap();
        let cards = parts[0].trim().chars().collect::<Vec<char>>();
        let card1 = CARDS[&cards[0]];
        let card2 = CARDS[&cards[1]];
        let card3 = CARDS[&cards[2]];
        let card4 = CARDS[&cards[3]];
        let card5 = CARDS[&cards[4]];

        Self {
            bid,
            card1,
            card2,
            card3,
            card4,
            card5
        }
    }

    pub fn cards(&self) -> Vec<i32> {
        let mut c = Vec::new();
        c.push(self.card1);
        c.push(self.card2);
        c.push(self.card3);
        c.push(self.card4);
        c.push(self.card5);
        c
    }

    pub fn count_of_card(&self, card: i32) -> i32 {
        let mut count = 0;
        if self.card1 == card { count += 1; }
        if self.card2 == card { count += 1; }
        if self.card3 == card { count += 1; }
        if self.card4 == card { count += 1; }
        if self.card5 == card { count += 1; }

        count
    }

    pub fn get_card_counts(&self) -> HashMap<i32, i32> {
        let mut hash = HashMap::new();
        hash.insert(self.card1, 1);

        match hash.get(&self.card2) {
            Some(n) => { hash.insert(self.card2, n+1); }
            None => { hash.insert(self.card2, 1); }
        }
        match hash.get(&self.card3) {
            Some(n) => { hash.insert(self.card3, n+1); }
            None => { hash.insert(self.card3, 1); }
        }
        match hash.get(&self.card4) {
            Some(n) => { hash.insert(self.card4, n+1); }
            None => { hash.insert(self.card4, 1); }
        }
        match hash.get(&self.card5) {
            Some(n) => { hash.insert(self.card5, n+1); }
            None => { hash.insert(self.card5, 1); }
        }
        hash
    }

    pub fn get_hand_type(&self) -> HandType {
        if self.is_five_of_kind() { return HandType::FiveOfAKind }
        if self.is_four_of_kind() { return HandType::FourOfAKind }
        if self.is_full_house() { return HandType::FullHouse }
        if self.is_three_of_a_kind() { return HandType::ThreeOfAKind }
        if self.is_two_pair() { return HandType::TwoPair }
        if self.is_pair() { return HandType::Pair }

        HandType::HighCard
    }
    pub fn is_five_of_kind(&self) -> bool {
        let counts = self.get_card_counts();
        counts.values().collect::<Vec<&i32>>()[0] == &5
    }

    pub fn is_four_of_kind(&self) -> bool {
        let card_counts = self.get_card_counts();
        let mut counts = card_counts
            .values()
            .collect::<Vec<&i32>>();

        counts.sort();
        counts.reverse();
        counts[0] == &4
    }

    pub fn is_full_house(&self) -> bool {
        let card_counts = self.get_card_counts();
        let mut counts = card_counts
            .values()
            .collect::<Vec<&i32>>();

        counts.sort();
        counts.reverse();
        counts[0] == &3 && counts[1] == &2
    }

    pub fn is_three_of_a_kind(&self) -> bool {
        let card_counts = self.get_card_counts();
        let mut counts = card_counts
            .values()
            .collect::<Vec<&i32>>();

        counts.sort();
        counts.reverse();
        counts[0] == &3
    }

    pub fn is_two_pair(&self) -> bool {
        let card_counts = self.get_card_counts();
        let mut counts = card_counts
            .values()
            .collect::<Vec<&i32>>();

        counts.sort();
        counts.reverse();
        counts[0] == &2 && counts[1] == &2
    }

    pub fn is_pair(&self) -> bool {
        let card_counts = self.get_card_counts();
        let mut counts = card_counts
            .values()
            .collect::<Vec<&i32>>();

        counts.sort();
        counts.reverse();
        counts[0] == &2
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_cmp = self.get_hand_type().cmp(&other.get_hand_type());
        if type_cmp != Ordering::Equal {
            return type_cmp;
        }

        let cards1 = self.cards();
        let cards2 = other.cards();
        for i in 0..5 {
            let card_cmp = cards1[i].cmp(&cards2[i]);
            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(self.cmp(other))
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let cards1 = self.cards();
        let cards2 = other.cards();
        for i in 0..5 {
            if cards1[i] != cards2[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Hand {}

impl Day {
    pub fn run(&self) {
        let reader = input_reader::InputReader;
        let mut hands = Vec::new();
        for line in reader.get_lines("/git/aoc23/src/days/day7/input") {
            hands.push(Hand::new(line));
        }

        hands.sort();

        let mut answer = 0;
        let mut n = 1;

        for h in hands {
            println!("n: {n} | bid: {0}", h.bid);
            answer += h.bid * n;
            n += 1;
        }

        println!("{answer}");
    }
}