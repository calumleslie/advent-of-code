use std::fs;
use std::error::Error;
use std::collections::HashMap;
use itertools::Itertools;

fn count_chars(word: &str) -> HashMap<char, u32> {
    let mut result: HashMap<char, u32> = HashMap::new();

    for c in word.chars() {
        let counter = result.entry(c).or_insert(0);
        *counter += 1;
    }

    return result;
}

pub fn part1() -> Result<i32, Box<Error>> {
    let file_contents = fs::read_to_string("inputs/day2-1")?;
    let lines: Vec<&str> = file_contents.lines().collect();

    let mut twos = 0;
    let mut threes = 0;
    for line in lines {
        let mut has_two = false;
        let mut has_three = false;
        for (_c, count) in count_chars(line) {
            has_two |= count == 2;
            has_three |= count == 3;
        }
        twos += has_two as i32;
        threes += has_three as i32;
    }

    return Ok(twos * threes);
}

pub fn part2() -> Result<String, Box<Error>> {
    let file_contents = fs::read_to_string("inputs/day2-1")?;
    let lines: Vec<&str> = file_contents.lines().collect();

    for (left, right) in lines.iter().tuple_combinations() {
        let mut combination = String::with_capacity(left.len() - 1);
        let mut differences = 0;

        for (lchar, rchar) in left.chars().zip(right.chars()) {
            if lchar != rchar {
                differences += 1;
            } else {
                combination.push(lchar);
            }
        }

        if differences == 1 {
            return Ok(combination);
        }
    }

    panic!("I didn't find anything");
}