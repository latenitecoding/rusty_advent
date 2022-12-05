use std::fs;

pub fn solve() -> (String, String) {
    let content = fs::read_to_string("inputs/y2022d01.txt").expect("file not found");
    (part_1(&content), part_2(&content))
}

/// PART 1 :
fn part_1(input: &str) -> String {
    format!("{}", "")
}

/// PART 2 :
fn part_2(input: &str) -> String {
    format!("{}", "")
}

fn parse_input(input: &str) -> Vec<&str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("")]
    fn test_part_1(#[case] input: &str) {
        assert_eq!(part_1(input), "");
    }

    #[rstest]
    #[case("")]
    fn test_part_2(#[case] input: &str) {
        assert_eq!(part_2(input), "");
    }
}
