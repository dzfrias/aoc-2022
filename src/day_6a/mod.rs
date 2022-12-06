use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .position(|chars| {
            if HashSet::<char>::from_iter(chars.into_iter().cloned()).len() == 4 {
                true
            } else {
                false
            }
        })
        .unwrap()
        + 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let inputs = [
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
        ];
        let expecteds = [7, 5, 6];
        for (input, expected) in inputs.into_iter().zip(expecteds) {
            assert_eq!(expected, solution(input));
        }
    }
}
