use std::collections::HashSet;

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

        // Get highest card
        let high_card = cards
            .iter()
            .map(|c| c.num)
            .collect::<Vec<u32>>()
            .iter()
            .max()
            .unwrap()
            .clone();

        Hand {
            rank: Hand::get_rank(&cards),
            cards,
            high_card,
        }
    }

    fn get_rank(cards: &[Card]) -> Ranks {
        // Filter repeated numbers and suits
        let (suits, nums) = cards.iter().fold(
            (HashSet::new(), HashSet::new()),
            |(mut suits, mut nums), card| {
                suits.insert(card.suit);
                nums.insert(card.num);
                (suits, nums)
            },
        );

        let rank = match (suits.len(), nums.len()) {
            (1, _) => Ranks::HighCard, // Flush case
            (_, 2) => Ranks::HighCard, // Poker or full
            (_, 3) => Ranks::HighCard, // Two pair or 3 of a kind
            (_, 4) => Ranks::OnePair,
            (_, _) => Ranks::HighCard, // Straig or High card
        };

        rank
    }

    fn flush_case(cards: &[Card]) -> Ranks {
        todo!()
    }

    fn poker_or_full(cards: &[Card]) -> Ranks {
        todo!()
    }

    fn two_pair_or_three(cards: &[Card]) -> Ranks {
        todo!()
    }

    fn straight_or_high_card(cards: &[Card]) -> Ranks {
        let nums = cards.iter().map(|c| c.num).collect::<Vec<u32>>();
        
        todo!()
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(round: &[&'a str]) -> Vec<&'a str> {
    // Get hands
    let hands = round.iter().fold(vec![], |mut hands, cards| {
        hands.push(Hand::new(cards));
        hands
    });

    compare_hands(hands);
    todo!("");
}

fn compare_hands(hands: Vec<Hand>) -> Hand<'_> {
    todo!("")
}
