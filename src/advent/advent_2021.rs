use std::fs;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

pub fn select_day(day: u32) -> Option<(String, String)> {
    match day {
        1 => Some(day_01::solve(
            &fs::read_to_string("inputs/y2021d01.txt").expect("file not found"),
        )),
        2 => Some(day_02::solve(
            &fs::read_to_string("inputs/y2021d02.txt").expect("file not found"),
        )),
        3 => Some(day_03::solve(
            &fs::read_to_string("inputs/y2021d03.txt").expect("file not found"),
        )),
        4 => Some(day_04::solve(
            &fs::read_to_string("inputs/y2021d04.txt").expect("file not found"),
        )),
        _ => None,
    }
}
