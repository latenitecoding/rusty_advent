use std::ops::RangeInclusive;

mod advent_2021;
mod advent_2022;

const DAYS: RangeInclusive<u32> = 1..=25;
const YEARS: RangeInclusive<u32> = 2015..=2022;

pub fn select_all_challenges() -> Vec<String> {
    YEARS
        .map(|year| select_challenges_from_year(year).into_iter())
        .flatten()
        .collect::<Vec<String>>()
}

pub fn select_challenges_from_year(year: u32) -> Vec<String> {
    DAYS.map(|day| select_challenge(year, day))
        .filter(|solution| solution.is_some())
        .map(|solution| solution.unwrap())
        .collect::<Vec<String>>()
}

pub fn select_challenges_with_day(day: u32) -> Vec<String> {
    YEARS
        .map(|year| select_challenge(year, day))
        .filter(|solution| solution.is_some())
        .map(|solution| solution.unwrap())
        .collect::<Vec<String>>()
}

pub fn select_challenge(year: u32, day: u32) -> Option<String> {
    let solution = match year {
        2021 => advent_2021::select_day(day),
        2022 => advent_2022::select_day(day),
        _ => None,
    };
    match solution {
        Some(solution) => Some(format!("year: {}, day: {} => {:?}", year, day, solution)),
        None => None,
    }
}
