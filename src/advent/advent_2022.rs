mod day_01;

pub fn select_day(day: u32) -> () {
    match day {
        1 => day_01::solve(),
        _ => println!("Sorry! No challenges on this day {}", day),
    }
}
