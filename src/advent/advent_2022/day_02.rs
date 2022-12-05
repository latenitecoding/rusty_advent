use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;

pub fn solve() -> (String, String) {
    let content = fs::read_to_string("inputs/y2022d02.txt").expect("file not found");
    (part_1(&content), part_2(&content))
}

/// Appreciative of your help yesterday, one Elf gives you an encrypted strategy
/// guide (your puzzle input) that they say will be sure to help you win. "The
/// first column is what your opponent is going to play: A for Rock, B for Paper,
/// and C for Scissors. The second column--" Suddenly, the Elf is called away
/// to help with someone's tent.
///
/// The second column, you reason, must be what you should play in response: X
/// for Rock, Y for Paper, and Z for Scissors. Winning every time would be
/// suspicious, so the responses must have been carefully chosen.
///
/// The winner of the whole tournament is the player with the highest score.
/// Your total score is the sum of your scores for each round. The score for a
/// single round is the score for the shape you selected (1 for Rock, 2 for
/// Paper, and 3 for Scissors) plus the score for the outcome of the round (0
/// if you lost, 3 if the round was a draw, and 6 if you won).
///
/// PART 1 : What would your total score be if everything goes exactly according
/// to your strategy guide?
fn part_1(input: &str) -> String {
    let total_score = parse_input(input)
        .into_iter()
        .map(|pair| (pair.0, Throw::from(pair.1)))
        .fold(0, |score, throws| score + scoring_of(&throws.0, &throws.1));
    format!("{}", total_score)
}

/// The Elf finishes helping with the tent and sneaks back over to you. "Anyway,
/// the second column says how the round needs to end: X means you need to lose,
/// Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
///
/// PART 2 : Following the Elf's instructions for the second column, what would
/// your total score be if everything goes exactly according to your strategy guide?
fn part_2(input: &str) -> String {
    let total_score = parse_input(input)
        .into_iter()
        .map(|pair| (pair.0, Throw::from((pair.0, Ordering::from(pair.1)))))
        .fold(0, |score, throws| score + scoring_of(&throws.0, &throws.1));
    format!("{}", total_score)
}

fn parse_input(input: &str) -> Vec<(Throw, Key)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            assert_eq!(
                parts.len(),
                2,
                "each line should include two space-delimited symbols"
            );
            (
                parts[0]
                    .parse::<Throw>()
                    .expect("parse error from line to Throw"),
                parts[1]
                    .parse::<Key>()
                    .expect("parse error from line to Key"),
            )
        })
        .collect::<Vec<(Throw, Key)>>()
}

fn scoring_of(opp_throw: &Throw, player_throw: &Throw) -> u32 {
    let throw_score = u32::from(player_throw);
    if player_throw > opp_throw {
        throw_score + 6
    } else if player_throw == opp_throw {
        throw_score + 3
    } else {
        throw_score
    }
}

#[derive(Debug)]
enum Key {
    X,
    Y,
    Z,
}

impl FromStr for Key {
    type Err = &'static str;

    fn from_str(key_str: &str) -> Result<Key, &'static str> {
        match key_str {
            "X" => Ok(Key::X),
            "Y" => Ok(Key::Y),
            "Z" => Ok(Key::Z),
            _ => Err("can only parse 'X', 'Y', and 'Z' as Keys"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Throw {
    type Err = &'static str;

    fn from_str(throw_str: &str) -> Result<Throw, &'static str> {
        match throw_str {
            "A" => Ok(Throw::Rock),
            "B" => Ok(Throw::Paper),
            "C" => Ok(Throw::Scissors),
            _ => Err("can only parse 'A', 'B', and 'C' as Throws"),
        }
    }
}

impl From<Key> for Throw {
    fn from(key: Key) -> Throw {
        match key {
            Key::X => Throw::Rock,
            Key::Y => Throw::Paper,
            Key::Z => Throw::Scissors,
        }
    }
}

impl From<(Throw, Ordering)> for Throw {
    fn from(pair: (Throw, Ordering)) -> Throw {
        match pair.1 {
            Ordering::Less => match pair.0 {
                Throw::Rock => Throw::Scissors,
                Throw::Paper => Throw::Rock,
                Throw::Scissors => Throw::Rock,
            },
            Ordering::Equal => pair.0,
            Ordering::Greater => match pair.0 {
                Throw::Rock => Throw::Paper,
                Throw::Paper => Throw::Scissors,
                Throw::Scissors => Throw::Rock,
            },
        }
    }
}

impl PartialOrd for Throw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (self_score, other_score) = (u32::from(self), u32::from(other));
        if self_score == other_score {
            Some(Ordering::Equal)
        } else if self_score % 3 + 1 == other_score {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl From<&Throw> for u32 {
    fn from(throw: &Throw) -> u32 {
        match throw {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }
}

impl From<Key> for Ordering {
    fn from(key: Key) -> Ordering {
        match key {
            Key::X => Ordering::Less,
            Key::Y => Ordering::Equal,
            Key::Z => Ordering::Greater,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("A Y\nB X\nC Z")]
    fn test_part_1(#[case] input: &str) {
        assert_eq!(part_1(input), "15");
    }

    #[rstest]
    #[case("A Y\nB X\nC Z")]
    fn test_part_2(#[case] input: &str) {
        assert_eq!(part_2(input), "12");
    }
}
