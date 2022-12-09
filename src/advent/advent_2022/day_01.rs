pub fn solve(input: &str) -> (String, String) {
    (part_1(&input), part_2(&input))
}

/// The jungle must be too overgrown and difficult to navigate in vehicles or
/// access from the air; the Elves' expedition traditionally goes on foot. As
/// your boats approach land, the Elves begin taking inventory of their supplies.
/// One important consideration is food - in particular, the number of Calories
/// each Elf is carrying (your puzzle input).
///
/// The Elves take turns writing down the number of Calories contained by the
/// various meals, snacks, rations, etc. that they've brought with them, one
/// item per line. Each Elf separates their own inventory from the previous
/// Elf's inventory (if any) by a blank line.
///
/// PART 1 : Find the Elf carrying the most Calories. How many total Calories
/// is that Elf carrying?
fn part_1(input: &str) -> String {
    let max_elf = parse_input(input)
        .into_iter()
        .max()
        .expect("input has no maximum");
    format!("{}", max_elf)
}

/// PART 2 : Find the top three Elves carrying the most Calories. How many
/// Calories are those Elves carrying in total?
fn part_2(input: &str) -> String {
    let mut elves = parse_input(input);
    elves.sort_by(|a, b| b.cmp(a));
    format!("{}", elves[..3].iter().sum::<u32>())
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .lines()
                .map(|line| {
                    line.parse::<u32>()
                        .expect("could not parse line as u32 of calories")
                })
                .sum::<u32>()
        })
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000")]
    fn test_part_1(#[case] input: &str) {
        assert_eq!(part_1(input), "24000");
    }

    #[rstest]
    #[case("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000")]
    fn test_part_2(#[case] input: &str) {
        assert_eq!(part_2(input), "45000");
    }
}
