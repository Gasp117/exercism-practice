use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Suits {
    Spade,
    Club,
    Heart,
    Diamond,
}

impl Suits {
    fn from_char(value: char) -> Option<Self> {
        match value {
            'S' => Some(Self::Spade),
            'C' => Some(Self::Club),
            'H' => Some(Self::Heart),
            'D' => Some(Self::Diamond),
            _ => None,
        }
    }
}

struct Card<'a> {
    num: u32,
    suit: Suits,
    card_str: &'a str,
}

impl<'a> Card<'a> {
    fn new(card_str: &'a str, len: usize) -> Self {
        // Get number and suit from char
        let (num, suit) = match len {
            2 => card_str
                .char_indices()
                .fold((0, None), |mut ret, (pos, c)| {
                    match pos {
                        0 => ret.0 = Card::get_card_number(c),
                        1 => ret.1 = Suits::from_char(c),
                        _ => (),
                    }
                    ret
                }),
            3 => (
                10u32,
                card_str
                    .char_indices()
                    .filter(|(pos, __)| *pos == 2)
                    .fold(None, |_, (_, c)| Suits::from_char(c)),
            ),
            _ => (0, None),
        };

        if suit.is_none() {
            println!("{card_str}");
        }
        // Create card
        Card {
            num,
            suit: suit.unwrap(),
            card_str,
        }
    }

    fn get_card_number(c: char) -> u32 {
        // If number, convert
        if let Some(num) = c.to_digit(10) {
            return num;
        }
        // Check letter
        match c {
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Ranks {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

struct Hand<'a> {
    cards: Vec<Card<'a>>,
    rank: Ranks,
    high_card: u32,
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str) -> Self {
        let cards = cards.split_whitespace().fold(vec![], |mut hand, card_str| {
            hand.push(Card::new(card_str, card_str.len()));
            hand
        });

        // Filter repeated numbers and suits
        let (suits, nums) = cards.iter().fold(
            (HashMap::new(), HashMap::new()),
            |(mut suits, mut nums), card| {
                *suits.entry(card.suit).or_insert(0) += 1;
                *nums.entry(card.num).or_insert(0) += 1;
                (suits, nums)
            },
        );

        let high_card = *nums.keys().max().unwrap();

        Hand {
            rank: Hand::get_rank(&suits, &nums),
            cards,
            high_card,
        }
    }

    fn get_rank(suits: &HashMap<Suits, i32>, nums: &HashMap<u32, i32>) -> Ranks {
        match (suits.len(), nums.len()) {
            (1, 5) => match Hand::check_straight(&mut nums.keys().collect::<Vec<&u32>>()) {
                true => Ranks::StraightFlush,
                false => Ranks::HighCard,
            }, // Flush straight case
            (1, _) => Ranks::Flush,
            (_, 2) => match nums.iter().any(|(_, v)| *v > 3) {
                true => Ranks::FourOfAKind,
                false => Ranks::FullHouse,
            },
            (_, 3) => match nums.iter().any(|(_, v)| *v > 2) {
                true => Ranks::ThreeOfAKind,
                false => Ranks::TwoPair,
            },
            (_, 4) => Ranks::OnePair,
            (_, 5) => match Hand::check_straight(&mut nums.keys().collect::<Vec<&u32>>()) {
                true => Ranks::Straight,
                false => Ranks::HighCard,
            }, // Straight case
            (_, _) => Ranks::HighCard,
        }
    }

    fn check_straight(nums: &mut [&u32]) -> bool {
        // First sort numbers
        nums.sort();
        // If fully contiguous, return true;
        if nums.windows(2).all(|p| *p[0] + 1 == *p[1]) {
            return true;
        }
        // Check A, 2, 3, 4, 5 case
        match (**nums.last().unwrap(), nums.iter().copied().sum::<u32>()) {
            (17, 14) => true,
            (_, _) => false,
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(round: &[&'a str]) -> Vec<&'a str> {
    // Get hands
    let mut hands = round.iter().fold(vec![], |mut hands, cards| {
        hands.push(Hand::new(cards));
        hands
    });

    compare_hands(&mut hands)
}

fn compare_hands<'a>(hands: &mut Vec<Hand<'a>>) -> Vec<&'a str> {
    // Sort by rank and 
    hands.sort_by_key(|h| h.rank);
    let ranks = hands.iter().fold(HashMap::new(), |mut ranks, h| {
        ranks.entry(h.rank.deref()).or_default(0) += 1;
        ranks
    });
    

    todo!("")
}
