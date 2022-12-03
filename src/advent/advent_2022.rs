mod day_01;
mod day_02;

pub fn select_day(day: u32) -> () {
    match day {
        1 => day_01::solve(),
        2 => day_02::solve(),
        _ => println!("Sorry! No challenges on this day {}", day),
    }
}
