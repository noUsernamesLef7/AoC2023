pub fn main(input: String) {
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
