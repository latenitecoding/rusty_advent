mod day_01;
mod day_02;
mod day_03;

pub fn select_day(day: u32) -> () {
    match day {
        1 => day_01::solve(),
        2 => day_02::solve(),
        3 => day_03::solve(),
        _ => println!("Sorry! No challenges on this day {}", day),
    }
}
