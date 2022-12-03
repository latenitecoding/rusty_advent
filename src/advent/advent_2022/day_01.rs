use std::fs::File;
use std::io::{prelude::*, BufReader};

/// The jungle must be too overgrown and difficult to navigate in vehicles or
/// access from the air; the Elves' expedition traditionally goes on foot. As
/// your boats approach land, the Elves begin taking inventory of their supplies.
/// One important consideration is food - in particular, the number of Calories
/// each Elf is carrying (your puzzle input). The Elves take turns writing down
/// the number of Calories contained by the various meals, snacks, rations, etc.
/// that they've brought with them, one item per line. Each Elf separates their
/// own inventory from the previous Elf's inventory (if any) by a blank line.
///
/// PART 1 : Find the Elf carrying the most Calories. How many total Calories
/// is that Elf carrying?
/// PART 2 : Find the top three Elves carrying the most Calories. How many 
/// Calories are those Elves carrying in total?
pub fn solve() -> () {
    let reader = BufReader::new(File::open("inputs/y2022d01.txt").expect("y2022d01.txt not found"));
    let mut elves = reader
        .lines()
        .fold(vec![0], |mut vec, line| -> Vec<i32> {
            let line = line.expect("read error");
            if line == "" {
                vec.push(0);
            } else {
                let n = vec.len();
                vec[n - 1] += line.parse::<i32>().expect("parse error");
            }
            vec
        })
        .into_iter()
        .map(|calories| Elf::from(calories))
        .collect::<Vec<Elf>>();
    elves.sort();
    println!(
        "year: 2022, day: 01 => ({:?}, {:?})",
        elves[0].calories,
        elves[..3].iter().map(|elf| elf.calories).sum::<i32>()
    );
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    pub calories: i32,
}

impl From<i32> for Elf {
    fn from(calories: i32) -> Elf {
        Elf { calories }
    }
}
