use std::fs;

/// As the submarine drops below the surface of the ocean, it automatically
/// performs a sonar sweep of the nearby sea floor. On a small screen, the sonar
/// sweep report (your puzzle input) appears: each line is a measurement of the
/// sea floor depth as the sweep looks further and further away from the submarine.
///
/// The first order of business is to figure out how quickly the depth
/// increases, just so you know what you're dealing with - you never know if the
/// keys will get carried into deeper water by an ocean current or a fish or something.
///
/// Considering every single measurement isn't as useful as you expected: there's
/// just too much noise in the data. Instead, consider sums of a three-measurement
/// sliding window.
pub fn solve() -> (String, String) {
    let content = fs::read_to_string("inputs/y2021d01.txt").expect("file not found");
    (part_1(content.as_str()), part_2(content.as_str()))
}

/// PART 1 : Count the number of times a depth measurement increases from the
/// previous measurement.
fn part_1(input: &str) -> String {
    let depths = parse_depths(input);
    let depth_increases = (1..depths.len()).fold(0, |count, idx| -> i32 {
        if depths[idx] > depths[idx - 1] {
            count + 1
        } else {
            count
        }
    });
    format!("{}", depth_increases)
}

/// PART 2 : Consider sums of a three-measurement sliding window. How many
/// sums are larger than the previous sum?
fn part_2(input: &str) -> String {
    let depths = parse_depths(input);
    let depth_increases = (3..depths.len()).fold(0, |count, n| -> i32 {
        if depths[n] > depths[n - 3] {
            count + 1
        } else {
            count
        }
    });
    format!("{}", depth_increases)
}

fn parse_depths(input: &str) -> Vec<Depth> {
    input
        .lines()
        .map(|line| Depth::from(line))
        .collect::<Vec<Depth>>()
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Depth {
    pub dist: i32,
}

impl From<&str> for Depth {
    fn from(line: &str) -> Depth {
        Depth {
            dist: line.parse::<i32>().expect("parse error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("199\n200\n208\n210\n200\n207\n240\n269\n260\n263")]
    fn test_part_1(#[case] input: &str) {
        assert_eq!(part_1(input), "7");
    }

    #[rstest]
    #[case("199\n200\n208\n210\n200\n207\n240\n269\n260\n263")]
    fn test_part_2(#[case] input: &str) {
        assert_eq!(part_2(input), "5");
    }
}
