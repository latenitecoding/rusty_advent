mod advent_2022;

pub fn select_all_challenges() -> () {}

pub fn select_all_challenges_from_year(year: u32) -> () {}

pub fn select_all_challenges_from_day(day: u32) -> () {}

pub fn select_challenge(year: u32, day: u32) -> () {
    match year {
        2022 => advent_2022::select_day(day),
        _ => println!("Sorry! No challenges for this year {}", year),
    }
}
