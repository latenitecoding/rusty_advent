use std::cmp::Ordering;
use std::fs;

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
/// The Elf finishes helping with the tent and sneaks back over to you. "Anyway,
/// the second column says how the round needs to end: X means you need to lose,
/// Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
pub fn solve() -> (String, String) {
    let content = fs::read_to_string("inputs/y2022d02.txt").expect("file not found");
    (part_1(content.as_str()), part_2(content.as_str()))
}

/// PART 1 : What would your total score be if everything goes exactly according
/// to your strategy guide?
fn part_1(input: &str) -> String {
    let total_score = parse_rounds(input)
        .iter()
        .fold(0, |score, round| score + scoring_of(round));
    format!("{}", total_score)
}

/// PART 2 : Following the Elf's instructions for the second column, what would
/// your total score be if everything goes exactly according to your strategy guide?
fn part_2(input: &str) -> String {
    let total_score = parse_rounds(input)
        .iter()
        .map(|round| Round {
            opp: round.opp,
            play: Throw::from((round.opp, Ordering::from(round.play))),
        })
        .fold(0, |score, round| score + scoring_of(&round));
    format!("{}", total_score)
}

fn parse_rounds(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|line| Round::from(line))
        .collect::<Vec<Round>>()
}

fn scoring_of(round: &Round) -> i32 {
    let score = match round.play {
        Throw::Rock => 1,
        Throw::Paper => 2,
        Throw::Scissors => 3,
    };
    if round.play > round.opp {
        score + 6
    } else if round.play == round.opp {
        score + 3
    } else {
        score
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Throw {
    fn from(throw: &str) -> Throw {
        match throw {
            "A" | "X" => Throw::Rock,
            "B" | "Y" => Throw::Paper,
            _ => Throw::Scissors,
        }
    }
}

impl From<Throw> for Ordering {
    fn from(throw: Throw) -> Ordering {
        match throw {
            Throw::Rock => Ordering::Less,
            Throw::Paper => Ordering::Equal,
            Throw::Scissors => Ordering::Greater,
        }
    }
}

impl From<(Throw, Ordering)> for Throw {
    fn from(pair: (Throw, Ordering)) -> Throw {
        match pair.1 {
            Ordering::Less => match pair.0 {
                Throw::Rock => Throw::Scissors,
                Throw::Paper => Throw::Rock,
                Throw::Scissors => Throw::Paper,
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
        match self {
            Throw::Rock => match other {
                Throw::Rock => Some(Ordering::Equal),
                Throw::Paper => Some(Ordering::Less),
                Throw::Scissors => Some(Ordering::Greater),
            },
            Throw::Paper => match other {
                Throw::Rock => Some(Ordering::Greater),
                Throw::Paper => Some(Ordering::Equal),
                Throw::Scissors => Some(Ordering::Less),
            },
            Throw::Scissors => match other {
                Throw::Rock => Some(Ordering::Less),
                Throw::Paper => Some(Ordering::Greater),
                Throw::Scissors => Some(Ordering::Equal),
            },
        }
    }
}

#[derive(Debug)]
struct Round {
    pub opp: Throw,
    pub play: Throw,
}

impl From<&str> for Round {
    fn from(line: &str) -> Round {
        let parts = line.split(" ").collect::<Vec<&str>>();
        Round {
            opp: Throw::from(parts[0]),
            play: Throw::from(parts[1]),
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
