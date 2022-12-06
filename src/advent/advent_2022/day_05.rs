pub fn solve(input: &str) -> (String, String) {
    (part_1(&input), part_2(&input))
}

/// The ship has a giant cargo crane capable of moving crates between stacks.
/// To ensure none of the crates get crushed or fall over, the crane operator
/// will rearrange them in a series of carefully-planned steps. After the
/// crates are rearranged, the desired crates will be at the top of each stack.
///
/// The Elves don't want to interrupt the crane operator during this delicate
/// procedure, but they forgot to ask her which crate will end up where, and they
/// want to be ready to unload them as soon as possible so they can embark.
/// PART 1 : After the rearrangement procedure completes, what crate ends up
/// on top of each stack?
fn part_1(input: &str) -> String {
    let mut stacks = parse_input_stacks(input);
    parse_input_queries(input).iter().for_each(|query| {
        (0..query.0).for_each(|_| {
            let ch = stacks[query.1 - 1].pop().expect("cargo stack is empty");
            stacks[query.2 - 1].push(ch);
        });
    });
    format!("{}", skim_top(&stacks))
}

/// Some mud was covering the writing on the side of the crane, and you quickly
/// wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.
///
/// The CrateMover 9001 is notable for many new and exciting features: air
/// conditioning, leather seats, an extra cup holder, and the ability to pick up
/// and move multiple crates at once.
///
/// PART 2 : After the rearrangement procedure completes, what crate ends up
/// on top of each stack?
fn part_2(input: &str) -> String {
    let mut stacks = parse_input_stacks(input);
    parse_input_queries(input).iter().for_each(|query| {
        let mut crane = (0..query.0)
            .map(|_| stacks[query.1 - 1].pop().expect("cargo stack is empty"))
            .collect::<Vec<char>>();
        while !crane.is_empty() {
            stacks[query.2 - 1].push(crane.pop().expect("crane is empty"));
        }
    });
    format!("{}", skim_top(&stacks))
}

fn parse_input_stacks(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n\n")
        .nth(0)
        .expect("input txt is empty")
        .lines()
        .rev()
        .skip(1)
        .fold(Vec::new(), |mut stacks, line| {
            while stacks.len() <= line.len() / 4 {
                stacks.push(Vec::new());
            }
            (2..line.len())
                .step_by(4)
                .filter(|n| !line[(n - 2)..=*n].trim().is_empty())
                .for_each(|n| {
                    let ch = line[(n - 2)..=n]
                        .chars()
                        .nth(1)
                        .expect("no letter ID for cargo");
                    stacks[n / 4].push(ch);
                });
            stacks
        })
}

fn parse_input_queries(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .split("\n\n")
        .nth(1)
        .expect("input txt has no queries")
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            assert_eq!(parts.len(), 6, "query is not properly formed");
            (
                parts[1]
                    .parse::<usize>()
                    .expect("parse error on cargo swap size"),
                parts[3]
                    .parse::<usize>()
                    .expect("parse error on origin stack"),
                parts[5]
                    .parse::<usize>()
                    .expect("parse error on dest stack"),
            )
        })
        .collect::<Vec<(usize, usize, usize)>>()
}

fn skim_top(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.last().expect("cargo stack is empty"))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2")]
    fn test_part_1(#[case] input: &str) {
        assert_eq!(part_1(input), "CMZ");
    }

    #[rstest]
    #[case("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2")]
    fn test_part_2(#[case] input: &str) {
        assert_eq!(part_2(input), "MCD");
    }
}
