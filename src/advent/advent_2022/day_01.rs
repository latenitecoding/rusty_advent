use std::fs::File;
use std::io::{prelude::*, BufReader};

/// The jungle must be too overgrown and difficult to navigate in vehicles or
/// access from the air; the Elves' expedition traditionally goes on foot. As
/// your boats approach land, the Elves begin taking inventory of their supplies.
/// One important consideration is food - in particular, the number of Calories
/// each Elf is carrying (your puzzle input).
///
/// The Elves take turns writing down the number of Calories contained by the
/// various meals, snacks, rations, etc. that they've brought with them, one
/// item per line. Each Elf separates their own inventory from the previous
/// Elf's inventory (if any) by a blank line.
///
/// PART 1 : Find the Elf carrying the most Calories. How many total Calories
/// is that Elf carrying?
/// PART 2 : Find the top three Elves carrying the most Calories. How many
/// Calories are those Elves carrying in total?
pub fn solve() -> () {
    let reader = BufReader::new(File::open("inputs/y2022d01.txt").expect("y2022d01.txt not found"));
    let mut elves = reader
        .lines()
        .fold(vec![Vec::new()], |mut vec, line| -> Vec<Vec<String>> {
            fold_lines(&mut vec, line.expect("read error"));
            vec
        })
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|line| line.parse::<i32>().expect("parse error"))
                .sum::<i32>()
        })
        .map(|calories| Elf::from(calories))
        .collect::<Vec<Elf>>();
    elves.sort_by(|a, b| b.cmp(a));
    println!(
        "year: 2022, day: 01 => ({:?}, {:?})",
        elves[0].calories,
        elves[..3].iter().map(|elf| elf.calories).sum::<i32>()
    );
}

fn fold_lines(vec: &mut Vec<Vec<String>>, line: String) -> () {
    if line == "" {
        vec.push(Vec::new());
    } else {
        vec.last_mut().expect("vec empty").push(line);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
        let elves = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .iter()
        .fold(vec![Vec::new()], |mut vec, line| -> Vec<Vec<String>> {
            fold_lines(&mut vec, line.to_string());
            vec
        })
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
        assert_eq!(elves, vec![6_000, 4_000, 11_000, 24_000, 10_000]);
    }

    #[test]
    fn part_02() {
        let mut elves = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .iter()
        .fold(vec![Vec::new()], |mut vec, line| -> Vec<Vec<String>> {
            fold_lines(&mut vec, line.to_string());
            vec
        })
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .map(|calories| Elf::from(calories))
        .collect::<Vec<Elf>>();
        elves.sort_by(|a, b| b.cmp(a));
        assert_eq!(
            elves[..3].iter().map(|elf| elf.calories).sum::<i32>(),
            45_000
        );
    }
}
