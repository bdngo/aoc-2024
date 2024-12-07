use std::{env, error::Error, fs};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let contents = fs::read_to_string(fname)?;

    let solution = day06::part2(contents);
    println!("{}", solution);

    Ok(())
}
