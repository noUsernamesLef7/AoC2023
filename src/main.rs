use clap::Parser;
use clap_stdin::FileOrStdin;

mod day1;
mod day2;
mod day6;
mod day7;

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
        1 => day1::main(args.input.to_string()),
        2 => day2::main(args.input.to_string()),
        6 => day6::main(args.input.to_string()),
        7 => day7::main(args.input.to_string()),
        _ => panic!("Day {} doesn't exist yet!", args.day),
    }
}
