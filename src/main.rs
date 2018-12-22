mod day1;

fn main() -> Result<(), Box<std::error::Error>> {
    let result = day1::part1()?;

    println!("{}", result);

    Ok(())
}
