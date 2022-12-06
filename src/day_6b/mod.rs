use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .position(|chars| {
            if HashSet::<char>::from_iter(chars.into_iter().cloned()).len() == 14 {
                true
            } else {
                false
            }
        })
        .unwrap()
        + 14
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
        let expecteds = [19, 23, 23];
        for (input, expected) in inputs.into_iter().zip(expecteds) {
            assert_eq!(expected, solution(input));
        }
    }
}
