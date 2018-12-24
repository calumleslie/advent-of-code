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

pub fn part1() -> Result<usize, Box<Error>> {
    let mut string = fs::read_to_string("inputs/day5-1")?;
    let mut removed = 0;
    loop {
        if !remove_reaction(&mut string) {
            println!("Removed {} reactions", removed);
            return Ok(string.len());
        }
        removed += 1;
    }
}
