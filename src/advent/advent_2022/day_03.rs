use std::fs::File;
use std::io::{prelude::*, BufReader};

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
/// PART 1 : Find the item type that appears in both compartments of each
/// rucksack. What is the sum of the priorities of those item types?
/// PART 2 : Find the item type that corresponds to the badges of each three-Elf
/// group. What is the sum of the priorities of those item types?
pub fn solve() -> () {
    let reader = BufReader::new(File::open("inputs/y2022d03.txt").expect("y2022d03.txt not found"));
    let rucksacks = reader
        .lines()
        .map(|line| Rucksack::from(line.expect("read error")))
        .collect::<Vec<Rucksack>>();
    let priorities = rucksacks
        .iter()
        .map(|sack| sack.intersect())
        .map(|item| priority_of(&item))
        .sum::<i32>();
    let badge_priorities = (3..rucksacks.len())
        .step_by(3)
        .map(|n| intersect_group(&rucksacks[(n - 3)..n]))
        .map(|badge| priority_of(&badge))
        .sum::<i32>();
    println!(
        "year: 2022, day: 03 => ({:?}, {:?})",
        priorities, badge_priorities
    );
}

fn priority_of(items: &String) -> i32 {
    let mut priority = 0;
    for ch in items.chars() {
        if 'a' <= ch && ch <= 'z' {
            priority += (ch as u32) - ('a' as u32) + 1;
        } else {
            priority += (ch as u32) - ('A' as u32) + 27;
        }
    }
    priority as i32
}

fn intersect_group(rucksacks: &[Rucksack]) -> String {
    let mut badge = String::new();
    for ch in rucksacks[0].left.chars() {
        if rucksacks[1].contains(ch) && rucksacks[2].contains(ch) && !badge.contains(ch) {
            badge.push(ch);
        }
    }
    for ch in rucksacks[0].right.chars() {
        if rucksacks[1].contains(ch) && rucksacks[2].contains(ch) && !badge.contains(ch) {
            badge.push(ch);
        }
    }
    assert_eq!(badge.len(), 1);
    badge
}

#[derive(Debug)]
struct Rucksack {
    pub left: String,
    pub right: String,
}

impl From<String> for Rucksack {
    fn from(line: String) -> Rucksack {
        let n = line.len();
        Rucksack {
            left: line[..(n / 2)].to_string(),
            right: line[(n / 2)..].to_string(),
        }
    }
}

impl Rucksack {
    fn contains(&self, ch: char) -> bool {
        self.left.contains(ch) || self.right.contains(ch)
    }

    fn intersect(&self) -> String {
        let mut item = String::new();
        for ch in self.left.chars() {
            if self.right.contains(ch) && !item.contains(ch) {
                item.push(ch);
            }
        }
        assert!(item.len() == 1);
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let items_vec = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .iter()
        .map(|line| Rucksack::from(line.to_string()))
        .map(|sack| sack.intersect())
        .collect::<Vec<String>>();
        assert_eq!(
            items_vec
                .iter()
                .map(|items| items.as_str())
                .collect::<Vec<&str>>(),
            vec!["p", "L", "P", "v", "t", "s"]
        );
        assert_eq!(
            items_vec
                .iter()
                .map(|items| priority_of(items))
                .collect::<Vec<i32>>(),
            vec![16, 38, 42, 22, 20, 19]
        );
    }

    #[test]
    fn sample_badges() {
        let group_1 = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ]
        .iter()
        .map(|line| Rucksack::from(line.to_string()))
        .collect::<Vec<Rucksack>>();
        let badge_1 = intersect_group(&group_1[..3]);
        assert_eq!(badge_1, "r".to_string());
        assert_eq!(priority_of(&badge_1), 18);
        let group_2 = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .iter()
        .map(|line| Rucksack::from(line.to_string()))
        .collect::<Vec<Rucksack>>();
        let badge_2 = intersect_group(&group_2[..3]);
        assert_eq!(badge_2, "Z".to_string());
        assert_eq!(priority_of(&badge_2), 52);
    }
}
