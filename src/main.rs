use clap::Parser;
use clap_stdin::FileOrStdin;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Parser, Debug)]
struct Args {
    /// Which day of the challenge to run
    #[arg(short, long)]
    day: u8,

    /// Optionally either puzzle input from stdin, or a file containing the puzzle input. If
    /// reading from stdin, use - for <Input> argument
    input: FileOrStdin<String>,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1(args.input.to_string()),
        2 => day2(args.input.to_string()),
        6 => day6(args.input.to_string()),
        7 => day7(args.input.to_string()),
        _ => panic!("Day {} doesn't exist yet!", args.day),
    }
}

fn day1(input: String) {
    let mut sum: u32 = 0;

    for line in input.lines() {
        // Get a vector of just the numerals
        let nums: Vec<&str> = line.matches(char::is_numeric).collect();

        // Place first and last elements of the vector into a new vector
        let digits: Vec<String> = vec![
            nums.first().unwrap().to_string(),
            nums.last().unwrap().to_string(),
        ];

        // Flatten the new vector into a single string and parse into u32
        let calibration_value = digits.join("").parse::<u32>().unwrap();

        sum += calibration_value;
    }

    println!("The sum of all calibration values is: {}", sum);
}

fn day2(input: String) {
    /*
    let mut sum: u32 = 0;
    let mut game_id: u32 = 1;

    for game in input.lines() {
        // Trim off the game number
        //
        // split on ; to get the rounds for each game
        //
        // for each round:
        //  use regex to get the number drawn for each color
        //
        //  if red, blue, green < targets
        //   sum += game_id
        //
        // game_id += 1
    }

    println!("The sum of all possible game ID's is: {}", sum);
    */
}

fn day6(input: String) {
    // Clean up input and break into Vec<Vec<u16>>
    let mut raw_data = input.replace("Time:", "");
    raw_data = raw_data.replace("Distance:", "");
    let mut race_data = Vec::new();
    for line in raw_data.lines() {
        let line_data: Vec<u16> = line
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u16>().unwrap())
            .collect();

        race_data.push(line_data);
    }

    let mut wins = vec![0; race_data[0].len()];
    for race in 0..race_data[0].len() {
        let race_length = race_data[0][race];
        let record = race_data[1][race];

        for time_held in 0..race_length + 1 {
            let distance = time_held * (race_length - time_held);

            if distance > record {
                wins[race] += 1;
            }
        }
    }

    println!(
        "The product of the number of winning strategies in each race is: {}",
        wins.iter().product::<usize>()
    );

    // Part 2
    let correct_length = race_data[0]
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let correct_record = race_data[1]
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let mut winning_strats = 0;
    for time_held in 0..correct_length + 1 {
        let distance = time_held * (correct_length - time_held);

        if distance > correct_record {
            winning_strats += 1;
        }
    }

    println!("There are {} ways to beat the actual race.", winning_strats);
}

fn day7(input: String) {
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
        score: ScoreType
    }
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            self.score.cmp(&other.score)
                .then(self.card1.cmp(&other.card1))
                .then(self.card2.cmp(&other.card2))
                .then(self.card3.cmp(&other.card3))
                .then(self.card4.cmp(&other.card4))
                .then(self.card5.cmp(&other.card5))
        }
    }

    // Turn input into a Vec<Hand>
    let mut hands = Vec::new();
    for line in input.lines() {
        let mut raw_data = line.split_whitespace();

        // map the chars for each card into Face enum
        let faces: Vec<Face> = raw_data.next()
            .unwrap()
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

        // Figure out ScoreType
        let counts = faces.clone()
            .into_iter()
            .counts();
        let score = match counts.values().max().unwrap() {
            5 => ScoreType::FiveOfAKind,
            4 => ScoreType::FourOfAKind,
            3 => match counts.values().find(|&&count| count == 2) {
                Some(_i) => ScoreType::FullHouse,
                None => ScoreType::ThreeOfAKind,
            },
            2 => match counts.values().duplicates().find(|&&count| count == 2) {
                Some(_i) => ScoreType::TwoPair,
                None => ScoreType::OnePair
            },
            1 => ScoreType::HighCard,
            _ => panic!("{:?} is not a valid value to compute score on!", counts.values()),
        };
        
        let hand = Hand {
            card1: faces[0],
            card2: faces[1],
            card3: faces[2],
            card4: faces[3],
            card5: faces[4],
            bid: raw_data.next()
                .unwrap()
                .parse()
                .unwrap(),
            score: score,
        };
        hands.push(hand);
    }

    // Sort all hands by score type, then by card1, card2...
    // This is managed in the Ord impl for Hand
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
