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
    match Args::parse() {
        Args { year: 0, day: 0 } => advent::select_all_challenges(),
        Args { year, day: 0 } => advent::select_all_challenges_from_year(year),
        Args { year: 0, day } => advent::select_all_challenges_from_day(day),
        Args { year, day } => advent::select_challenge(year, day),
    }
}
