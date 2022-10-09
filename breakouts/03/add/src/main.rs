use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        return ExitCode::FAILURE
    }

    let sum = args.iter().skip(1).fold(0, |acc, x| acc + x.parse::<i32>().unwrap());
    println!("Sum: {sum}");

    ExitCode::SUCCESS
}
