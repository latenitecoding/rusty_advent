pub fn solve(input: &str) -> (String, String) {
    (part_1(&input), part_2(&input))
}

/// Space needs to be cleared before the last supplies can be unloaded from the
/// ships, and so several Elves have been assigned the job of cleaning up sections
/// of the camp. Every section has a unique ID number, and each Elf is assigned a
/// range of section IDs.
///
/// However, as some of the Elves compare their section assignments with each
/// other, they've noticed that many of the assignments overlap. To try to quickly
/// find overlaps and reduce duplicated effort, the Elves pair up and make a big
/// list of the section assignments for each pair (your puzzle input).
///
/// PART 1 : In how many assignment pairs does one range fully contain the other?
fn part_1(input: &str) -> String {
    let num_redundant_pairs =
        parse_input(input)
            .iter()
            .fold(0, |num_pairs, pair| match pair.0.intersect(&pair.1) {
                Some(intersect) => {
                    if pair.0 == intersect || pair.1 == intersect {
                        num_pairs + 1
                    } else {
                        num_pairs
                    }
                }
                None => num_pairs,
            });
    format!("{}", num_redundant_pairs)
}

/// It seems like there is still quite a bit of duplicate work planned. Instead, the
/// Elves would like to know the number of pairs that overlap at all.
///
/// PART 2 : In how many assignment pairs do the ranges overlap?
fn part_2(input: &str) -> String {
    let num_overlapping_pairs =
        parse_input(input)
            .iter()
            .fold(0, |num_pairs, pair| match pair.0.intersect(&pair.1) {
                Some(_) => num_pairs + 1,
                None => num_pairs,
            });
    format!("{}", num_overlapping_pairs)
}

fn parse_input(input: &str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(",").collect::<Vec<&str>>();
            (Assignment::from(parts[0]), Assignment::from(parts[1]))
        })
        .collect::<Vec<(Assignment, Assignment)>>()
}

#[derive(Debug, PartialEq)]
struct Assignment {
    pub left: u32,
    pub right: u32,
}

impl From<&str> for Assignment {
    fn from(line: &str) -> Assignment {
        let parts = line.split("-").collect::<Vec<&str>>();
        assert_eq!(
            parts.len(),
            2,
            "each assignment should include two '-' delimited u32s"
        );
        Assignment {
            left: parts[0]
                .parse::<u32>()
                .expect("parse error on assignment left"),
            right: parts[1]
                .parse::<u32>()
                .expect("parse error on assignment right"),
        }
    }
}

impl Assignment {
    fn intersect(&self, other: &Assignment) -> Option<Assignment> {
        if self.right < other.left || other.right < self.left {
            None
        } else if other.left <= self.left && self.right <= other.right {
            Some(Assignment {
                left: self.left,
                right: self.right,
            })
        } else if self.left <= other.left && other.right <= self.right {
            Some(Assignment {
                left: other.left,
                right: other.right,
            })
        } else if self.left < other.left && self.right <= other.right {
            Some(Assignment {
                left: other.left,
                right: self.right,
            })
        } else {
            Some(Assignment {
                left: self.left,
                right: other.right,
            })
        }
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
