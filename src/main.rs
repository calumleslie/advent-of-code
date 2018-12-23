mod day1;

use std::env;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let arg_values: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

    match &arg_values[1..] {
        ["day1", "part1"] => println!("{}", day1::part1()?),
        ["day1", "part2"] => println!("{}", day1::part2()?),
        _ => panic!("Don't know how to {:?}", args)
    }

    Ok(())
}
