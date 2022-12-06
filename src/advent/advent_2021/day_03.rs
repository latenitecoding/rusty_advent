use std::fs;

pub fn solve() -> (String, String) {
    let content = fs::read_to_string("inputs/y2021d03.txt").expect("file not found");
    (part_1(&content), part_2(&content))
}

/// The diagnostic report (your puzzle input) consists of a list of binary
/// numbers which, when decoded properly, can tell you many useful things about
/// the conditions of the submarine. The first parameter to check is the power consumption.
///
/// You need to use the binary numbers in the diagnostic report to generate
/// two new binary numbers (called the gamma rate and the epsilon rate). The power
/// consumption can then be found by multiplying the gamma rate by the epsilon rate.
///
/// Each bit in the gamma rate can be determined by finding the most common bit in
/// the corresponding position of all numbers in the diagnostic report.
///
/// PART 1 : Use the binary numbers in your diagnostic report to calculate the
/// gamma rate and epsilon rate, then multiply them together. What is the power
/// consumption of the submarine?
fn part_1(input: &str) -> String {
    let diagnostics = parse_input(input);
    let m = diagnostics[0].len();
    let (gamma_rate, epsilon_rate) = (0..m)
        .map(|k| {
            let (num_zeroes, num_ones) = diagnostics
                .iter()
                .map(|diagnostic| diagnostic.digit_at(k))
                .fold((0, 0), |(num_zeroes, num_ones), bit| {
                    if bit == 0 {
                        (num_zeroes + 1, num_ones)
                    } else {
                        (num_zeroes, num_ones + 1)
                    }
                });
            if num_ones >= num_zeroes {
                (1, 0)
            } else {
                (0, 1)
            }
        })
        .fold((0, 0), |(gamma_rate, epsilon_rate), bit_pair| {
            (
                (gamma_rate << 1) + bit_pair.0,
                (epsilon_rate << 1) + bit_pair.1,
            )
        });
    format!("{}", gamma_rate * epsilon_rate)
}

/// Next, you should verify the life support rating, which can be determined by
/// multiplying the oxygen generator rating by the CO2 scrubber rating.
///
/// Both the oxygen generator rating and the CO2 scrubber rating are values that
/// can be found in your diagnostic report - finding them is the tricky part. Both
/// values are located using a similar process that involves filtering out values
/// until only one remains. Before searching for either rating value, start with
/// the full list of binary numbers from your diagnostic report and consider just
/// the first bit of those numbers. Then:
/// - Keep only numbers selected by the bit criteria for the type of rating value
///   for which you are searching. Discard numbers which do not match the bit criteria.
/// - If you only have one number left, stop; this is the rating value for which
///   you are searching.
/// - Otherwise, repeat the process, considering the next bit to the right.
///
/// The bit criteria depends on which type of rating value you want to find:
/// - To find oxygen generator rating, determine the most common value (0 or 1) in
///   the current bit position, and keep only numbers with that bit in that position.
///   If 0 and 1 are equally common, keep values with a 1 in the position being considered.
/// - To find CO2 scrubber rating, determine the least common value (0 or 1) in the
///   current bit position, and keep only numbers with that bit in that position.
///   If 0 and 1 are equally common, keep values with a 0 in the position being considered.
///
/// PART 2 : Use the binary numbers in your diagnostic report to calculate the
/// oxygen generator rating and CO2 scrubber rating, then multiply them together.
/// What is the life support rating of the submarine?
fn part_2(input: &str) -> String {
    let diagnostics = parse_input(input);
    let m = diagnostics[0].len();
    let mut prefixes = vec![0u32; (1 << (m + 1)) - 1];
    (0..m)
        .map(|k| {
            diagnostics
                .iter()
                .map(move |diagnostic| (1 << (k + 1)) | diagnostic.prefix(k))
        })
        .flatten()
        .for_each(|prefix| prefixes[prefix] += 1);
    let (oxygen_rate, co2_rate) = (0..m).fold((0, 0), |(mut oxygen_rate, mut co2_rate), k| {
        let oxygen_prefix = ((1 << k) | oxygen_rate) << 1;
        let co2_prefix = ((1 << k) | co2_rate) << 1;
        if prefixes[oxygen_prefix | 1] >= prefixes[oxygen_prefix] {
            oxygen_rate = (oxygen_rate << 1) | 1;
        } else {
            oxygen_rate <<= 1;
        }
        if prefixes[co2_prefix | 1] == 0
            || prefixes[co2_prefix | 1] >= prefixes[co2_prefix] && prefixes[co2_prefix] > 0
        {
            co2_rate <<= 1;
        } else {
            co2_rate = (co2_rate << 1) | 1;
        }
        (oxygen_rate, co2_rate)
    });
    format!("{}", oxygen_rate * co2_rate)
}

fn parse_input(input: &str) -> Vec<Diagnostic> {
    input
        .lines()
        .map(|line| Diagnostic::from(line))
        .collect::<Vec<Diagnostic>>()
}

#[derive(Debug)]
struct Diagnostic {
    pub length: usize,
    pub report: u32,
}

impl Diagnostic {
    fn digit_at(&self, k: usize) -> u32 {
        self.report & (1 << (self.length - 1 - k))
    }

    fn len(&self) -> usize {
        self.length
    }

    fn prefix(&self, k: usize) -> usize {
        (self.report as usize) >> (self.length - 1 - k)
    }
}

impl From<&str> for Diagnostic {
    fn from(line: &str) -> Diagnostic {
        let mut length = 0;
        let mut report = 0;
        line.chars().for_each(|ch| {
            report <<= 1;
            if ch == '1' {
                report += 1;
            }
            length += 1;
        });
        Diagnostic { length, report }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010")]
    fn test_part_1(#[case] input: &str) {
        assert_eq!(part_1(input), "198");
    }

    #[rstest]
    #[case("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010")]
    fn test_part_2(#[case] input: &str) {
        assert_eq!(part_2(input), "230");
    }
}
