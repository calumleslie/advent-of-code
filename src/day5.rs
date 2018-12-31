use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::min;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::iter::Iterator;

lazy_static! {
    static ref RE: Regex = build_reaction_regex();
}

fn build_reaction_regex() -> Regex {
    let mut pattern: String = String::new();
    let mut first = true;
    pattern.push('(');
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        if !first {
            pattern.push('|');
        }
        first = false;
        pattern.push(c);
        pattern.push(c.to_ascii_uppercase());
        pattern.push('|');
        pattern.push(c.to_ascii_uppercase());
        pattern.push(c);
    }
    pattern.push(')');
    return Regex::new(&pattern).unwrap();
}

fn remove_reaction(value: &str) -> String {
    return RE.replace_all(value, "").into_owned();
}

fn fully_reacted_size(input: &str) -> usize {
    let mut result = input.to_string();
    let mut last_len = result.len();
    loop {
        result = remove_reaction(&result);
        if result.len() == last_len {
            return result.len();
        }
        last_len = result.len();
    }
}

pub fn part1() -> Result<usize, Box<Error>> {
    let input = fs::read_to_string("inputs/day5-1")?;

    Ok(fully_reacted_size(&input))
}

pub fn part2() -> Result<usize, Box<Error>> {
    let input = fs::read_to_string("inputs/day5-1")?;

    let unit_types: BTreeSet<char> = input.chars().map(|c| c.to_ascii_lowercase()).collect();

    let mut min_size = usize::max_value();

    for unit_type in unit_types {
        let unit_removed = input.replace(&[unit_type, unit_type.to_ascii_uppercase()][..], "");
        let size = fully_reacted_size(&unit_removed);
        min_size = min(size, min_size);

        println!("{} => {}", unit_type, size);
    }

    Ok(min_size)
}
