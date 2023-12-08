use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
    possible: bool,
}
impl Round {
    fn new(round_string: &str) -> Round {
        // Compile regex used to extract color and count
        static RE_COLOR: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?P<n>\d+) (?P<type>red|green|blue)").unwrap());

        let (mut red, mut blue, mut green) = (0, 0, 0);

        for cube in round_string.split(',') {
            let (_full, [count, color]) = RE_COLOR.captures(cube).unwrap().extract();
            match color {
                "red" => red = count.parse::<usize>().unwrap(),
                "green" => green = count.parse::<usize>().unwrap(),
                "blue" => blue = count.parse::<usize>().unwrap(),
                _ => panic!("{} is not a valid cube color!", color),
            }
        }
        Round {
            red: red,
            green: green,
            blue: blue,
            possible: Self::possible(red, green, blue),
        }
    }
    fn possible(red: usize, green: usize, blue: usize) -> bool {
        if red > 12 || green > 13 || blue > 14 {
            return false;
        }
        true
    }
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
    possible: bool,
}
impl Game {
    fn new(game_string: &str) -> Game {
        let rounds = game_string
            .split(';')
            .map(|round| Round::new(round))
            .collect();
        Game {
            possible: Self::possible(&rounds),
            rounds: rounds,
        }
    }
    fn possible(rounds: &Vec<Round>) -> bool {
        !rounds.iter().any(|&round| round.possible == false)
    }
}

pub fn main(input: String) {
    // Use a regular expression to remove the game prefixes
    let re = Regex::new(r"Game.*: ").unwrap();
    let raw_data = re.replace_all(input.as_str(), "");

    // Get a Vec<Game> of all games
    let games: Vec<Game> = raw_data.lines().map(|line| Game::new(line)).collect();

    println!(
        "The sum of all possible games is: {}",
        games
            .iter()
            .enumerate()
            .filter(|(_i, game)| game.possible)
            .map(|(i, _game)| i + 1)
            .sum::<usize>()
    );
}
