mod day1;
mod day2;
mod day3;
mod day4;

use std::env;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let arg_values: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

    match &arg_values[1..] {
        ["day1", "part1"] => println!("{}", day1::part1()?),
        ["day1", "part2"] => println!("{}", day1::part2()?),
        ["day2", "part1"] => println!("{}", day2::part1()?),
        ["day2", "part2"] => println!("{}", day2::part2()?),
        ["day3", "part1"] => println!("{}", day3::part1()?),
        ["day3", "part2"] => println!("{:?}", day3::part2()?),
        ["day4", "part1"] => println!("{}", day4::part1()?),
        ["day4", "part2"] => println!("{}", day4::part2()?),
        _ => panic!("Don't know how to {:?}", args),
    }

    Ok(())
}
