mod day_01;

pub fn select_day(day: u32) -> Option<(String, String)> {
    match day {
        1 => Some(day_01::solve()),
        _ => None,
    }
}
