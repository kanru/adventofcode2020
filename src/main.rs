use anyhow::Error;
use argh::FromArgs;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod day2;

#[derive(FromArgs)]
/// Advent of code 2020
struct AdventOfCode {
    /// set the day for the input
    #[argh(option)]
    day: u32,
    /// set the part for the input
    #[argh(option)]
    part: u32,
    /// the input file containing the puzzle
    #[argh(positional)]
    input_file: String,
}

fn main() -> Result<(), Error> {
    color_backtrace::install();

    let aoc: AdventOfCode = argh::from_env();
    match (aoc.day, aoc.part) {
	(1, 1) => day1_part1(&aoc.input_file),
	(1, 2) => day1_part2(&aoc.input_file),
	(2, 1) => day2::part1(&aoc.input_file),
	(2, 2) => day2::part2(&aoc.input_file),
	_ => Ok(())
    }
}


fn day1_part1(input_file: &str) -> Result<(), Error> {
    let input = BufReader::new(File::open(input_file)?);
    let mut seen = HashSet::new();

    for line in input.lines() {
        let num: u64 = line?.parse()?;
        let oth = 2020 - num;
        if seen.contains(&oth) {
            println!("Answer is {}", num * oth);
	    break;
        } else {
            seen.insert(num);
        }
    }

    Ok(())
}

fn day1_part2(input_file: &str) -> Result<(), Error> {
    let input = BufReader::new(File::open(input_file)?);
    let mut seen = HashSet::new();

    for line in input.lines() {
	let num: u64 = line?.parse()?;
	seen.insert(num);
    }

    for &first in &seen {
	if first >= 2020 {
	    continue;
	}
	let target = 2020 - first;
	for &second in &seen {
	    if second >= target {
		continue;
	    }
	    let third = target - second;
	    if seen.contains(&third) {
		println!("Found {} * {} * {} = {}",
			 first, second, third, first * second * third);
		return Ok(());
	    }
	}
    }
    
    Ok(())
}
