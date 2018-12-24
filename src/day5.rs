use std::cmp::min;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::iter::Iterator;

fn char_pairs_with_indexes<'a>(input: &'a str) -> Box<Iterator<Item = (usize, (char, char))> + 'a> {
    let iter1 = input.chars();
    let mut iter2 = input.chars();
    iter2.next();
    Box::new(iter1.zip(iter2).enumerate())
}

fn reactable(pair: (char, char)) -> bool {
    (pair.0.is_uppercase() != pair.1.is_uppercase())
        && (pair.0.to_ascii_lowercase() == pair.1.to_ascii_lowercase())
}

fn remove_reaction(value: &mut String) -> bool {
    let reaction = char_pairs_with_indexes(value).find(|pair| reactable(pair.1));
    if reaction.is_none() {
        return false;
    }

    let index = reaction.expect("").0;
    value.replace_range(index..(index + 2), "");
    true
}

fn fully_reacted_size(input: &str) -> usize {
    let mut result = input.to_string();
    loop {
        if !remove_reaction(&mut result) {
            return result.len();
        }
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
