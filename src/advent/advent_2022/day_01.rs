use std::fs;

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
pub fn solve() -> (String, String) {
    let content = fs::read_to_string("inputs/y2022d01.txt").expect("file not found");
    (part_1(content.as_str()), part_2(content.as_str()))
}

/// PART 1 : Find the Elf carrying the most Calories. How many total Calories
/// is that Elf carrying?
fn part_1(input: &str) -> String {
    let elf_calories = parse_elves(input)
        .iter()
        .max()
        .expect("at least one elf")
        .calories;
    format!("{}", elf_calories)
}

/// PART 2 : Find the top three Elves carrying the most Calories. How many
/// Calories are those Elves carrying in total?
fn part_2(input: &str) -> String {
    let mut elves = parse_elves(input);
    elves.sort_by(|a, b| b.cmp(a));
    format!("{}", elves[..3].iter().map(|elf| elf.calories).sum::<i32>())
}

fn parse_elves(input: &str) -> Vec<Elf> {
    input
        .split("\n")
        .fold(vec![Vec::new()], |mut vec, line| -> Vec<Vec<&str>> {
            fold_lines_into_vec(&mut vec, line);
            vec
        })
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|line| line.parse::<i32>().expect("parse error"))
                .sum::<i32>()
        })
        .map(|calories| Elf::from(calories))
        .collect::<Vec<Elf>>()
}

fn fold_lines_into_vec<'a>(vec: &mut Vec<Vec<&'a str>>, line: &'a str) -> () {
    if line == "" {
        vec.push(Vec::new());
    } else {
        vec.last_mut().expect("vec empty").push(line);
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    pub calories: i32,
}

impl From<i32> for Elf {
    fn from(calories: i32) -> Elf {
        Elf { calories }
    }
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
