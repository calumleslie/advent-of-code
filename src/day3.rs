use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Claim {
    n: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn from_str(input: &str) -> Claim {
        let mut cols = input.split(' ');
        let n_col = cols.next().expect("Bad line");
        cols.next(); // @
        let pos_col = cols.next().expect("Bad line");
        let size_col = cols.next().expect("Bad line");

        // #123
        let n: u32 = u32::from_str(&n_col[1..]).expect("n not a u32");

        // 123,456:
        let mut pos_bits = pos_col.split(',');
        let left_bit = pos_bits.next().expect("Can't find left");
        let top_bit = pos_bits.next().expect("can't find right");
        let left = u32::from_str(left_bit).expect("left not a u32");
        let top = u32::from_str(&top_bit[..top_bit.len() - 1]).expect("top not a u32");

        // 123x456
        let mut size_bits = size_col.split('x');
        let width_bit = size_bits.next().expect("Can't find width");
        let height_bit = size_bits.next().expect("Can't find height");
        let width = u32::from_str(width_bit).expect("width not a u32");
        let height = u32::from_str(height_bit).expect("height not a u32");

        Claim {
            n,
            left,
            top,
            width,
            height,
        }
    }

    fn right(&self) -> u32 {
        self.left + self.width
    }

    fn bottom(&self) -> u32 {
        self.top + self.height
    }

    fn in_coord(&self, x: u32, y: u32) -> bool {
        self.left <= x && self.right() > x && self.top <= y && self.bottom() > y
    }

    fn overlaps(&self, other: &Claim) -> bool {
        let h_overlap = (self.left <= other.left && self.right() >= other.left)
            || (other.left <= self.left && other.right() >= self.left);

        let v_overlap = (self.top <= other.top && self.bottom() >= other.top)
            || (other.top <= self.top && other.bottom() >= self.top);

        h_overlap && v_overlap
    }
}

fn read_file() -> Result<Vec<Claim>, Box<Error>> {
    let file_contents = fs::read_to_string("inputs/day3-1")?;
    Ok(file_contents.lines().map(Claim::from_str).collect())
}

pub fn part1() -> Result<i32, Box<Error>> {
    let claims = read_file()?;

    let right_extent = claims.iter().map(Claim::right).max().expect("No data");
    let bottom_extent = claims.iter().map(Claim::bottom).max().expect("No data");

    let mut multiple = 0;
    for x in 0..=right_extent {
        for y in 0..=bottom_extent {
            let claim_count = claims.iter().filter(|c| c.in_coord(x, y)).count();
            if claim_count > 1 {
                multiple += 1;
            }
        }
    }

    Ok(multiple)
}

pub fn part2() -> Result<Claim, Box<Error>> {
    let claims = read_file()?;
    let mut has_overlap: HashSet<Claim> = HashSet::new();

    for (left, right) in claims.iter().tuple_combinations() {
        if left.overlaps(right) {
            has_overlap.insert(*left);
            has_overlap.insert(*right);
        }
    }

    for claim in claims {
        if !has_overlap.contains(&claim) {
            return Ok(claim);
        }
    }

    panic!("No result found");
}
