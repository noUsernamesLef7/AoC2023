use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
enum Face {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
enum ScoreType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    card1: Face,
    card2: Face,
    card3: Face,
    card4: Face,
    card5: Face,
    bid: usize,
    score: ScoreType,
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            .then(self.card1.cmp(&other.card1))
            .then(self.card2.cmp(&other.card2))
            .then(self.card3.cmp(&other.card3))
            .then(self.card4.cmp(&other.card4))
            .then(self.card5.cmp(&other.card5))
    }
}
impl Hand {
    fn new(characters: &str, bid: usize) -> Hand {
        let cards: Vec<Face> = characters
            .chars()
            .map(|character| match character {
                'A' => Face::Ace,
                'K' => Face::King,
                'Q' => Face::Queen,
                'J' => Face::Jack,
                'T' => Face::Ten,
                '9' => Face::Nine,
                '8' => Face::Eight,
                '7' => Face::Seven,
                '6' => Face::Six,
                '5' => Face::Five,
                '4' => Face::Four,
                '3' => Face::Three,
                '2' => Face::Two,
                _ => panic!("The character {} does not match any known card!", character),
            })
            .collect();

        Hand {
            card1: cards[0],
            card2: cards[1],
            card3: cards[2],
            card4: cards[3],
            card5: cards[4],
            bid: bid,
            score: Self::calculate_score(cards),
        }
    }

    fn calculate_score(cards: Vec<Face>) -> ScoreType {
        // Get a HashMap containing the number of each type of card in the hand
        let counts = cards.into_iter().counts();

        match counts.values().max().unwrap() {
            5 => ScoreType::FiveOfAKind,
            4 => ScoreType::FourOfAKind,
            3 => match counts.values().find(|&&count| count == 2) {
                Some(_i) => ScoreType::FullHouse,
                None => ScoreType::ThreeOfAKind,
            },
            2 => match counts.values().duplicates().find(|&&count| count == 2) {
                Some(_i) => ScoreType::TwoPair,
                None => ScoreType::OnePair,
            },
            1 => ScoreType::HighCard,
            _ => panic!(
                "{:?} is not a valid value to compute score on!",
                counts.values()
            ),
        }
    }
}

pub fn main(input: String) {

    // Turn input into a Vec<Hand>
    let mut hands = Vec::new();
    for line in input.lines() {

        // Break input line into a list of chars and the bid
        let mut raw_data = line.split_whitespace();
        let faces = raw_data.next()
            .unwrap();
        let bid: usize = raw_data.next()
            .unwrap()
            .parse()
            .unwrap();
        
        // Create a new Hand and push it to the list of all hands
        hands.push(Hand::new(faces, bid));
    }

    // Sort all hands by score type, then by card1, card2...
    // This is managed in the Ord impl for Hand
    // Then reverse the resulting sorted Vector to get the final multipliers 
    hands.sort();
    hands.reverse();

    // Calculate total_winnings
    let mut total_winnings = 0;
    hands.iter()
        .enumerate()
        .for_each(|(i, hand)| {
            total_winnings += (i + 1) * hand.bid
        });

    println!("Total Winnings: {}",total_winnings);
}
