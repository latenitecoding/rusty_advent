mod advent_2022;

pub fn select_all_challenges() -> () {
    for year in 2015..=2022 {
        select_all_challenges_from_year(year);
    }
}

pub fn select_all_challenges_from_year(year: u32) -> () {
    for day in 1..=25 {
        select_challenge(year, day);
    }
}

pub fn select_all_challenges_from_day(day: u32) -> () {
    for year in 2015..=2022 {
        select_challenge(year, day);
    }
}

pub fn select_challenge(year: u32, day: u32) -> () {
    match year {
        2022 => advent_2022::select_day(day),
        _ => println!("Sorry! No challenges for this year {}", year),
    }
}
