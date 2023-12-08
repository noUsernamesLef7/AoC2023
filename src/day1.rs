pub fn main(input: String) {
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
