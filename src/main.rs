use clap::Parser;

mod advent;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Advent Year
    #[arg(short, long, default_value_t = 0)]
    year: u32,

    /// Advent Day
    #[arg(short, long, default_value_t = 0)]
    day: u32,
}

fn main() {
    let solutions = match Args::parse() {
        Args { year: 0, day: 0 } => advent::select_all_challenges(),
        Args { year, day: 0 } => advent::select_challenges_from_year(year),
        Args { year: 0, day } => advent::select_challenges_with_day(day),
        Args { year, day } => match advent::select_challenge(year, day) {
            Some(solution) => vec![solution],
            None => Vec::new(),
        },
    };
    if solutions.is_empty() {
        println!("Sorry! No solutions...");
    } else {
        solutions
            .iter()
            .rev()
            .for_each(|solution| println!("{}", solution));
    }
}
