use std::fmt;
use std::ops::RangeInclusive;

mod advent_2021;
mod advent_2022;

const DAYS: RangeInclusive<u32> = 1..=25;
const YEARS: RangeInclusive<u32> = 2015..=2022;

pub fn select_all_challenges() -> Vec<Solution> {
    YEARS
        .flat_map(|year| select_challenges_from_year(year).into_iter())
        .collect::<Vec<Solution>>()
}

pub fn select_challenges_from_year(year: u32) -> Vec<Solution> {
    DAYS
        .filter_map(|day| select_challenge(year, day))
        .collect::<Vec<Solution>>()
}

pub fn select_challenges_with_day(day: u32) -> Vec<Solution> {
    YEARS
        .filter_map(|year| select_challenge(year, day))
        .collect::<Vec<Solution>>()
}

pub fn select_challenge(year: u32, day: u32) -> Option<Solution> {
    let solve = match year {
        2021 => advent_2021::select_day(day),
        2022 => advent_2022::select_day(day),
        _ => None,
    };
    match solve {
        Some((part1, part2)) => Some(Solution {
            year,
            day,
            part1,
            part2,
        }),
        None => None,
    }
}

#[derive(Debug)]
pub struct Solution {
    year: u32,
    day: u32,
    part1: String,
    part2: String,
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Solution {{year = {}}}, {{day = {}}} => ({}, {})", self.year, self.day, self.part1, self.part2)
    }
}
