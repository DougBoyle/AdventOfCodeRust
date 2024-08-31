use std::{cmp::Ordering, collections::HashMap, str::FromStr};

fn main() {
    Part1::process();
    Part2::process();
}

static PART1_CARDS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
static PART2_CARDS: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

trait Part: Sized {
    fn new() -> Self;

    // Instance methods to avoid dead code warnings on unused members of type Part
    fn get_card_ranks(&self) -> &'static [char; 13];

    fn get_counts(&self, values: [char; 5]) -> HashMap<char, usize>;

    fn process() {
        let mut bids: Vec<Bid<Self>> = rust_aoc::read_input(7).map(|s| s.parse()).map(Result::unwrap).collect();
        bids.sort_by(|first, second| first.hand.cmp(&second.hand));
        let total: usize = bids.iter().rev().enumerate().map(|(i, bid)| (i+1)*bid.bid).sum();
        // 1. 253910319
        // 2. 254083736
        println!("Total: {total}"); 
    }

    fn get_type(&self, values: [char; 5]) -> HandType {
        let counts = self.get_counts(values);
        match counts.values().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => if counts.values().any(|c| *c == 2) { HandType::FullHouse } else { HandType::ThreeOfAKind },
            2 => if counts.values().filter(|c| **c == 2).count() == 2 { HandType::TwoPair } else { HandType::OnePair },
            1 => HandType::HighCard,
            n => panic!("Invalid hand, has {n} repeating cards?")
        }
    }
}

struct Part1;

impl Part for Part1 {
    fn new() -> Self {
        Part1
    }

    fn get_card_ranks(&self) -> &'static [char; 13] {
        &PART1_CARDS
    }

    fn get_counts(&self, values: [char; 5]) -> HashMap<char, usize> {
        let mut result = HashMap::new();
        for c in values {
            result.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }
        result
    }
}

struct Part2;

impl Part for Part2 {
    fn new() -> Self {
        Part2
    }

    fn get_card_ranks(&self) -> &'static [char; 13] {
        &PART2_CARDS
    }

    fn get_counts(&self, values: [char; 5]) -> HashMap<char, usize> {
        let mut result = HashMap::new();
        for c in values {
            result.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }
        if let Some(jokers) = result.remove(&'J') {
            // If all 5 cards were jokers, we just put them all back
            let max_key = result.iter().max_by_key(|(_, val)| **val).map_or('J', |(key, _)| *key);
            result.entry(max_key).and_modify(|count| *count += jokers).or_insert(jokers);
        }
        result
    }
}

struct Bid<Ranking: Part> {
    hand: Hand<Ranking>,
    bid: usize,
}

impl<Ranking: Part> FromStr for Bid<Ranking> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let (cards, bid) = rust_aoc::split_in_two(s, ' ');
        let hand = cards.parse().unwrap();
        let bid = bid.parse().unwrap();
        let bid = Bid { hand, bid };
        Ok(bid)
    }
}

struct Hand<Ranking: Part> {
    cards: [char; 5],
    hand_type: HandType,
    ranking: Ranking,
}

impl<Ranking: Part> Hand<Ranking> {
    fn get_card_rank(c: &char, ranking: &[char; 13]) -> usize {
        ranking.iter().position(|card| card == c).unwrap()
    }

    fn get_card_ranks(&self) -> impl Iterator<Item=usize> + '_ {
        let ranking = self.ranking.get_card_ranks();
        self.cards.iter().map(|c| Self::get_card_rank(c, ranking))
    }
}

impl<Ranking: Part> FromStr for Hand<Ranking> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self,()> {
        let ranking = Ranking::new();
        let cards = s.chars().collect::<Vec<_>>().try_into().unwrap();
        let hand_type = ranking.get_type(cards);
        let hand = Hand::<Ranking> { cards, hand_type, ranking };
        Ok(hand)
    }
}

impl<Ranking: Part> Ord for Hand<Ranking> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                self.get_card_ranks().zip(other.get_card_ranks())
                    .map(|(i1, i2)| i1.cmp(&i2))
                    .filter(|order| *order != Ordering::Equal)
                    .next().unwrap()
            },
            order => order
        }
    }
}

impl<Ranking: Part> PartialOrd for Hand<Ranking> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Ranking: Part> PartialEq for Hand<Ranking> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl<Ranking: Part> Eq for Hand<Ranking> {}

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}


