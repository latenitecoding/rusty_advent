use std::collections::HashSet;
use std::str::Chars;

pub fn solve(input: &str) -> (String, String) {
    (part_1(&input), part_2(&input))
}

/// Each rucksack has two large compartments. All items of a given type are meant
/// to go into exactly one of the two compartments. The Elf that did the packing
/// failed to follow this rule for exactly one item type per rucksack.
///
/// The Elves have made a list of all of the items currently in each rucksack
/// (your puzzle input), but they need your help finding the errors. Every item
/// type is identified by a single lowercase or uppercase letter (that is, a and
/// A refer to different types of items).
///
/// The list of items for each rucksack is given as characters all on a single
/// line. A given rucksack always has the same number of items in each of its
/// two compartments, so the first half of the characters represent items in the
/// first compartment, while the second half of the characters represent items in
/// the second compartment.
///
/// To help prioritize item rearrangement, every item type can be converted to a priority:
/// - Lowercase item types a through z have priorities 1 through 26.
/// - Uppercase item types A through Z have priorities 27 through 52.
///
/// PART 1 : Find the item type that appears in both compartments of each
/// rucksack. What is the sum of the priorities of those item types?
fn part_1(input: &str) -> String {
    let priority_sum = parse_input(input)
        .into_iter()
        .map(|sack| sack.intersect_compartments())
        .map(|item| priority_of(&item))
        .sum::<u32>();
    format!("{}", priority_sum)
}

/// For safety, the Elves are divided into groups of three. Every Elf carries a
/// badge that identifies their group. For efficiency, within each group of three
/// Elves, the badge is the only item type carried by all three Elves. That is, if
/// a group's badge is item type B, then all three Elves will have item type B
/// somewhere in their rucksack, and at most two of the Elves will be carrying any
/// other item type.
///
/// Additionally, nobody wrote down which item type corresponds to each group's
/// badges. The only way to tell which item type is the right one is by finding
/// the one item type that is common between all three Elves in each group.
///
/// Every set of three lines in your list corresponds to a single group, but
/// each group can have a different badge item type.
///
/// PART 2 : Find the item type that corresponds to the badges of each three-Elf
/// group. What is the sum of the priorities of those item types?
fn part_2(input: &str) -> String {
    let rucksacks = parse_input(input);
    let priority_sum = (3..=rucksacks.len())
        .step_by(3)
        .map(|n| intersect_group(&rucksacks[(n - 3)..n]))
        .map(|badge| priority_of(&badge))
        .sum::<u32>();
    format!("{}", priority_sum)
}

fn parse_input(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| Rucksack::from(line))
        .collect::<Vec<Rucksack>>()
}

fn intersect_group(group: &[Rucksack]) -> String {
    let mut init: HashSet<char> = HashSet::new();
    group[0].chars().for_each(|ch| {
        init.insert(ch);
    });
    group[1..]
        .iter()
        .fold(init, |set, sack| -> HashSet<char> {
            let mut inter: HashSet<char> = HashSet::new();
            sack.chars().filter(|ch| set.contains(ch)).for_each(|ch| {
                inter.insert(ch);
            });
            inter
        })
        .iter()
        .collect::<String>()
}

fn priority_of(items: &str) -> u32 {
    items
        .chars()
        .map(|ch| {
            if 'a' <= ch && ch <= 'z' {
                (ch as u32) - ('a' as u32) + 1
            } else if 'A' <= ch && ch <= 'Z' {
                (ch as u32) - ('A' as u32) + 27
            } else {
                0
            }
        })
        .sum::<u32>()
}

#[derive(Debug)]
struct Rucksack<'a> {
    pub items: &'a str,
}

impl<'a> From<&'a str> for Rucksack<'a> {
    fn from(items: &'a str) -> Rucksack<'a> {
        Rucksack { items }
    }
}

impl Rucksack<'_> {
    fn intersect_compartments(&self) -> String {
        let mut left_set: HashSet<char> = HashSet::new();
        for ch in self.left() {
            left_set.insert(ch);
        }
        self.right()
            .filter(|ch| left_set.contains(ch))
            .collect::<HashSet<char>>()
            .iter()
            .collect::<String>()
    }

    fn chars(&self) -> Chars<'_> {
        self.items.chars()
    }

    fn left(&self) -> Chars<'_> {
        self.items[..(self.items.len() / 2)].chars()
    }

    fn right(&self) -> Chars<'_> {
        self.items[(self.items.len() / 2)..].chars()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw")]
    fn test_part_1(#[case] input: &str) {
        assert_eq!(part_1(input), "157");
    }

    #[rstest]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw")]
    fn test_part_2(#[case] input: &str) {
        assert_eq!(part_2(input), "70");
    }
}
