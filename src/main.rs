use clap::Parser;
use clap_stdin::FileOrStdin;

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

    println!("The product of the number of winning strategies in each race is: {}", wins.iter().product::<usize>());
}
