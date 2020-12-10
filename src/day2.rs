use anyhow::{Context, Error};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

pub(crate) fn part1(input_file: &str) -> Result<(), Error> {
    day2(input_file, &policy_part1)
}

pub(crate) fn part2(input_file: &str) -> Result<(), Error> {
    day2(input_file, &policy_part2)
}

fn day2(input_file: &str, policy: &dyn Fn(usize, usize, char, &str) -> bool) -> Result<(), Error> {
    let input = BufReader::new(File::open(input_file)?);
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): (\w+)$")?;

    let mut valid = 0;
    for line in input.lines() {
	let line = line?;
	let caps = re.captures(&line).context("Failed to match anything")?;
	let min: usize = caps.get(1).unwrap().as_str().parse()?;
	let max: usize = caps.get(2).unwrap().as_str().parse()?;
	let chr: char = caps.get(3).unwrap().as_str().parse()?;
	let pas = caps.get(4).unwrap().as_str();

	if policy(min, max, chr, pas) {
	    valid += 1;
	}
    }
    println!("There is total {} valid password", valid);

    Ok(())
}

fn policy_part1(min: usize, max: usize, chr: char, pas: &str) -> bool {
    let count = pas.chars().filter(|c| *c == chr).count();
    return count >= min && count <= max;
}

fn policy_part2(pos1: usize, pos2: usize, chr: char, pas: &str) -> bool {
    let pos1_match = pas.chars().nth(pos1 - 1).unwrap() == chr;
    let pos2_match = pas.chars().nth(pos2 - 1).unwrap() == chr;
    return pos1_match ^ pos2_match;
}
