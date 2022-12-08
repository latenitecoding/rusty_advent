use std::collections::HashSet;

pub fn solve(input: &str) -> (String, String) {
    (part_1(&input), part_2(&input))
}

/// As you move through the dense undergrowth, one of the Elves gives you a
/// handheld device. He says that it has many fancy features, but the most
/// important one to set up right now is the communication system.
///
/// However, because he's heard you have significant experience dealing with
/// signal-based systems, he convinced the other Elves that it would be okay to
/// give you their one malfunctioning device - surely you'll have no problem
/// fixing it.
///
/// To be able to communicate with the Elves, the device needs to lock on to
/// their signal. The signal is a series of seemingly-random characters that the
/// device receives one at a time.
///
/// To fix the communication system, you need to add a subroutine to the device
/// that detects a start-of-packet marker in the datastream. In the protocol
/// being used by the Elves, the start of a packet is indicated by a sequence of
/// four characters that are all different.
///
/// The device will send your subroutine a datastream buffer (your puzzle input);
/// your subroutine needs to identify the first position where the four most
/// recently received characters were all different. Specifically, it needs to
/// report the number of characters from the beginning of the buffer to the end
/// of the first such four-character marker.
///
/// PART 1 : How many characters need to be processed before the first
/// start-of-packet marker is detected?
fn part_1(input: &str) -> String {
    let target_len = 4;
    assert!(
        input.len() >= target_len,
        "input should include at least four characters"
    );
    let start_of_packet_marker = (target_len..=input.len())
        .find(|n| {
            input[(n - target_len)..*n]
                .chars()
                .collect::<HashSet<char>>()
                .len()
                == target_len
        })
        .expect("no solution");
    format!("{}", start_of_packet_marker)
}

/// Your device's communication system is correctly detecting packets, but
/// still isn't working. It looks like it also needs to look for messages.
///
/// A start-of-message marker is just like a start-of-packet marker, except it
/// consists of 14 distinct characters rather than 4.
///
/// PART 2 : How many characters need to be processed before the first
/// start-of-message marker is detected?
fn part_2(input: &str) -> String {
    let target_len = 14;
    assert!(
        input.len() >= target_len,
        "input should include at least four characters"
    );
    let start_of_message_marker = (target_len..=input.len())
        .find(|n| {
            input[(n - target_len)..*n]
                .chars()
                .collect::<HashSet<char>>()
                .len()
                == target_len
        })
        .expect("no solution");
    format!("{}", start_of_message_marker)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7")]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", "5")]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", "6")]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10")]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11")]
    fn test_part_1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(part_1(input), expected);
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "19")]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", "23")]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", "23")]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "29")]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "26")]
    fn test_part_2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(part_2(input), expected);
    }
}
