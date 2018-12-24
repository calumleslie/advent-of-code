use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter::Iterator;
use std::str::FromStr;

fn read_file() -> Result<Vec<i32>, Box<Error>> {
    let file_contents = fs::read_to_string("inputs/day1-1")?;

    // Yes, yes, I know, the expect
    let result: Vec<i32> = file_contents
        .lines()
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

pub fn part2() -> Result<i32, Box<Error>> {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut frequency = 0;
    for value in read_file()?.iter().cycle() {
        frequency = frequency + value;
        if seen.contains(&frequency) {
            return Ok(frequency);
        }
        seen.insert(frequency);
    }
    unreachable!()
}
