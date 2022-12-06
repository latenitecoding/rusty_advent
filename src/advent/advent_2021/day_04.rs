use std::fs;

pub fn solve() -> (String, String) {
    let content = fs::read_to_string("inputs/y2021d04.txt").expect("file not found");
    (part_1(&content), part_2(&content))
}

/// Bingo is played on a set of boards each consisting of a 5x5 grid of numbers.
/// Numbers are chosen at random, and the chosen number is marked on all boards on
/// which it appears. (Numbers may not appear on all boards.) If all numbers in
/// any row or any column of a board are marked, that board wins. (Diagonals don't count.)
///
/// The submarine has a bingo subsystem to help passengers (currently, you and
/// the giant squid) pass the time. It automatically generates a random order in
/// which to draw numbers and a random set of boards (your puzzle input).
///
/// The score of the winning board can now be calculated. Start by finding the
/// sum of all unmarked numbers on that board. Then, multiply that sum by the
/// number that was just called when the board won to get the final score.
///
/// PART 1 : To guarantee victory against the giant squid, figure out which
/// board will win first. What will your final score be if you choose that board?
fn part_1(input: &str) -> String {
    let queries = parse_input_queries(input);
    let mut boards = parse_input_boards(input);
    let final_score = queries.iter().fold(0, |final_score, q| {
        if final_score > 0 {
            return final_score;
        }
        boards.iter_mut().fold(0, |score, board| {
            if score > 0 {
                return score;
            }
            board.update(*q);
            if board.has_won() {
                q * board.sum()
            } else {
                0
            }
        })
    });
    format!("{}", final_score)
}

/// On the other hand, it might be wise to try a different strategy: let the
/// giant squid win. You aren't sure how many bingo boards a giant squid could
/// play at once, so rather than waste time counting its arms, the safe thing to
/// do is to figure out which board will win last and choose that one. That way,
/// no matter which boards it picks, it will win for sure.
///
/// PART 2 : Figure out which board will win last. Once it wins, what would
/// its final score be?
fn part_2(input: &str) -> String {
    let queries = parse_input_queries(input);
    let mut boards = parse_input_boards(input);
    let final_score = queries.iter().fold(0, |final_score, q| {
        boards.iter_mut().fold(final_score, |score, board| {
            if board.has_won() {
                return score;
            }
            board.update(*q);
            if board.has_won() {
                q * board.sum()
            } else {
                score
            }
        })
    });
    format!("{}", final_score)
}

fn parse_input_queries(input: &str) -> Vec<u32> {
    input
        .lines()
        .nth(0)
        .expect("no initial query provided")
        .split(",")
        .map(|part| part.parse::<u32>().expect("parse error on query"))
        .collect::<Vec<u32>>()
}

fn parse_input_boards(input: &str) -> Vec<BingoBoard> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    (7..=lines.len())
        .step_by(6)
        .map(|n| BingoBoard::from(&lines[(n - 5)..n]))
        .collect::<Vec<BingoBoard>>()
}

#[derive(Debug)]
struct BingoBoard {
    pub board: [[u32; 6]; 6],
}

impl From<&[&str]> for BingoBoard {
    fn from(lines: &[&str]) -> BingoBoard {
        let mut board = [[0u32; 6]; 6];
        lines
            .iter()
            .zip(1..=5)
            .map(|(line, row)| {
                line.split(" ")
                    .filter(|part| !part.is_empty())
                    .zip(1..=5)
                    .map(move |(part, col)| {
                        (row, col, part.parse::<u32>().expect("parse error on board"))
                    })
            })
            .flatten()
            .for_each(|(row, col, num)| {
                board[row][col] = num;
                board[0][col] += 1;
                board[row][0] += 1;
                board[0][0] += num;
            });
        BingoBoard { board }
    }
}

impl BingoBoard {
    fn has_won(&self) -> bool {
        for i in 1..=5 {
            if self.board[i][0] == 0 || self.board[0][i] == 0 {
                return true;
            }
        }
        false
    }

    fn sum(&self) -> u32 {
        self.board[0][0]
    }

    fn update(&mut self, num: u32) -> () {
        for row in 1..=5 {
            for col in 1..=5 {
                if self.board[row][col] == num {
                    self.board[0][col] -= 1;
                    self.board[row][0] -= 1;
                    self.board[0][0] -= num;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7")]
    fn test_part_1(#[case] input: &str) {
        assert_eq!(part_1(input), "4512");
    }

    #[rstest]
    #[case("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7")]
    fn test_part_2(#[case] input: &str) {
        assert_eq!(part_2(input), "1924");
    }
}
