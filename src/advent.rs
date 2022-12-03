mod advent_2021;
mod advent_2022;

pub fn select_all_challenges() -> Vec<String> {
    (2015..=2022)
        .map(|year| select_all_challenges_from_year(year).into_iter())
        .flatten()
        .collect::<Vec<String>>()
}

pub fn select_all_challenges_from_year(year: u32) -> Vec<String> {
    (1..=25)
        .map(|day| select_challenge(year, day))
        .filter(|solution| solution.is_some())
        .map(|solution| solution.unwrap())
        .collect::<Vec<String>>()
}

pub fn select_all_challenges_from_day(day: u32) -> Vec<String> {
    (2015..=2022)
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
