mod day_01;
mod day_02;
mod day_03;
mod day_04;

pub fn select_day(day: u32) -> Option<(String, String)> {
    match day {
        1 => Some(day_01::solve()),
        2 => Some(day_02::solve()),
        3 => Some(day_03::solve()),
        4 => Some(day_04::solve()),
        _ => None,
    }
}
