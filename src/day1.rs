use std::fs;
use std::str::FromStr;
use std::iter::Iterator;
use std::error::Error;

fn read_file() -> Result<Vec<i32>, Box<Error>> {
    let file_contents = fs::read_to_string("inputs/day1-1")?;

    // Yes, yes, I know, the expect
    let result: Vec<i32> = file_contents.lines()
        .map(|line| i32::from_str(line).expect("Faild to read line"))
        .collect();

    Ok(result)
}

pub fn part1() -> Result<i32, Box<Error>> {
    let mut frequency = 0;
    for value in read_file()? {
        frequency = frequency + value;
    }
    Ok(frequency)
}