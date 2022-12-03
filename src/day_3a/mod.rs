use std::collections::HashSet;

pub fn solution(input: &str) -> u32 {
    input.lines().fold(0, |accum, line| {
        let (first, second) = line.split_at(line.len() / 2);
        let hash1 = first.chars().collect::<HashSet<char>>();
        let hash2 = second.chars().collect::<HashSet<char>>();

        let val = {
            let intersect = hash1.intersection(&hash2).next().unwrap();
            if intersect.is_uppercase() {
                *intersect as u32 - 38
            } else {
                *intersect as u32 - 96
            }
        };
        accum + val
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, solution(input));
    }
}
