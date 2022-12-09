use std::ops::RangeInclusive;

pub fn solve(input: &str) -> (String, String) {
    (part_1(&input), part_2(&input))
}

/// Space needs to be cleared before the last supplies can be unloaded from the
/// ships, and so several Elves have been assigned the job of cleaning up sections
/// of the camp. Every section has a unique ID number, and each Elf is assigned a
/// range of section IDs.
///
/// However, as some of the Elves compare their section assignments with each
/// r2, they've noticed that many of the assignments overlap. To try to quickly
/// find overlaps and reduce duplicated effort, the Elves pair up and make a big
/// list of the section assignments for each pair (your puzzle input).
///
/// PART 1 : In how many assignment pairs does one range fully contain the r2?
fn part_1(input: &str) -> String {
    let num_redundant_pairs = parse_input(input)
        .into_iter()
        .fold(0, |num_pairs, (r1, r2)| {
            match intersect_range(r1.clone(), r2.clone()) {
                Some(intersect) => {
                    if r1 == intersect || r2 == intersect {
                        num_pairs + 1
                    } else {
                        num_pairs
                    }
                }
                None => num_pairs,
            }
        });
    format!("{}", num_redundant_pairs)
}

/// It seems like there is still quite a bit of duplicate work planned. Instead, the
/// Elves would like to know the number of pairs that overlap at all.
///
/// PART 2 : In how many assignment pairs do the ranges overlap?
fn part_2(input: &str) -> String {
    let num_overlapping_pairs = parse_input(input)
        .into_iter()
        .fold(0, |num_pairs, (r1, r2)| match intersect_range(r1, r2) {
            Some(_) => num_pairs + 1,
            None => num_pairs,
        });
    format!("{}", num_overlapping_pairs)
}

fn parse_input(input: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(",").collect::<Vec<&str>>();
            assert_eq!(
                parts.len(),
                2,
                "could not parse line as two comma-delimited assignments"
            );
            (parse_range(parts[0]), parse_range(parts[1]))
        })
        .collect::<Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>>()
}

fn parse_range(input: &str) -> RangeInclusive<usize> {
    let parts = input.split("-").collect::<Vec<&str>>();
    assert_eq!(
        parts.len(),
        2,
        "could not parse line as two hyphen-delimited intervals"
    );
    parts[0]
        .parse::<usize>()
        .expect("could not parse line as u32 of start")
        ..=parts[1]
            .parse::<usize>()
            .expect("could not parse line as u32 of end")
}

fn intersect_range<T: PartialOrd>(
    r1: RangeInclusive<T>,
    r2: RangeInclusive<T>,
) -> Option<RangeInclusive<T>> {
    let (r1_start, r1_end) = r1.into_inner();
    let (r2_start, r2_end) = r2.into_inner();
    if r1_end < r2_start || r2_end < r1_start {
        None
    } else if r2_start <= r1_start && r1_end <= r2_end {
        Some(r1_start..=r1_end)
    } else if r1_start <= r2_start && r2_end <= r1_end {
        Some(r2_start..=r2_end)
    } else if r1_start < r2_start && r1_end <= r2_end {
        Some(r2_start..=r1_end)
    } else {
        Some(r1_start..=r2_end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8")]
    fn test_part_1(#[case] input: &str) {
        assert_eq!(part_1(input), "2");
    }

    #[rstest]
    #[case("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8")]
    fn test_part_2(#[case] input: &str) {
        assert_eq!(part_2(input), "4");
    }
}
