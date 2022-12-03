mod day_01;
mod day_02;

pub fn select_day(day: u32) -> Option<(String, String)> {
    match day {
        1 => Some(day_01::solve()),
        2 => Some(day_02::solve()),
        _ => None,
    }
}
