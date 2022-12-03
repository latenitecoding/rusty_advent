use std::fs;

pub fn solve() -> (String, String) {
    let content = fs::read_to_string("inputs/y2021d02.txt").expect("file not found");
    (part_1(content.as_str()), part_2(content.as_str()))
}

/// Now, you need to figure out how to pilot this thing. It seems like the
/// submarine can take a series of commands like forward 1, down 2, or up 3:
/// - forward X increases the horizontal position by X units.
/// - down X increases the depth by X units.
/// - up X decreases the depth by X units.
///
/// Your horizontal position and depth both start at 0.
///
/// PART 1 : Calculate the horizontal position and depth you would have after
/// following the planned course. What do you get if you multiply your final
/// horizontal position by your final depth?
fn part_1(input: &str) -> String {
    let (pos, depth) =
        parse_commands(input)
            .iter()
            .fold((0, 0), |(pos, depth), command| -> (i32, i32) {
                match command.op {
                    SubOp::Forward => (pos + command.dist, depth),
                    SubOp::Down => (pos, depth + command.dist),
                    SubOp::Up => (pos, depth - command.dist),
                }
            });
    format!("{}", pos * depth)
}

/// In addition to horizontal position and depth, you'll also need to track
/// a third value, aim, which also starts at 0. The commands also mean something
/// entirely different than you first thought:
/// - down X increases your aim by X units.
/// - up X decreases your aim by X units.
/// - forward X does two things:
///     - It increases your horizontal position by X units.
///     - It increases your depth by your aim multiplied by X.
///
/// PART 2 : Using this new interpretation of the commands, calculate the
/// horizontal position and depth you would have after following the planned
/// course. What do you get if you multiply your final horizontal position by
/// your final depth?
fn part_2(input: &str) -> String {
    let (pos, depth, _) = parse_commands(input).iter().fold(
        (0, 0, 0),
        |(pos, depth, aim), command| -> (i32, i32, i32) {
            match command.op {
                SubOp::Forward => (pos + command.dist, depth + aim * command.dist, aim),
                SubOp::Down => (pos, depth, aim + command.dist),
                SubOp::Up => (pos, depth, aim - command.dist),
            }
        },
    );
    format!("{}", pos * depth)
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| Command::from(line))
        .collect::<Vec<Command>>()
}

#[derive(Debug)]
enum SubOp {
    Forward,
    Down,
    Up,
}

impl From<&str> for SubOp {
    fn from(line: &str) -> SubOp {
        match line {
            "forward" => SubOp::Forward,
            "down" => SubOp::Down,
            _ => SubOp::Up,
        }
    }
}

#[derive(Debug)]
struct Command {
    pub op: SubOp,
    pub dist: i32,
}

impl From<&str> for Command {
    fn from(line: &str) -> Command {
        let parts = line.split(" ").collect::<Vec<&str>>();
        Command {
            op: SubOp::from(parts[0]),
            dist: parts[1].parse::<i32>().expect("parse error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2")]
    fn test_part_1(#[case] input: &str) {
        assert_eq!(part_1(input), "150");
    }

    #[rstest]
    #[case("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2")]
    fn test_part_2(#[case] input: &str) {
        assert_eq!(part_2(input), "900");
    }
}
