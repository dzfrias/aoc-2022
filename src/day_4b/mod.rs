use std::collections::HashSet;

pub fn solution(input: &str) -> u32 {
    input.lines().fold(0, |accum, line| {
        let [s1, s2] = <[&str; 2]>::try_from(line.split(",").collect::<Vec<_>>()).unwrap();
        let range1 = <[&str; 2]>::try_from(s1.split("-").collect::<Vec<_>>())
            .unwrap()
            .map(|s| s.parse::<u32>().unwrap());
        let range2 = <[&str; 2]>::try_from(s2.split("-").collect::<Vec<_>>())
            .unwrap()
            .map(|s| s.parse::<u32>().unwrap());
        let hash1 = (range1[0]..=range1[1]).collect::<HashSet<_>>();
        let hash2 = (range2[0]..=range2[1]).collect::<HashSet<_>>();
        if hash1.intersection(&hash2).count() > 0 {
            accum + 1
        } else {
            accum
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(4, solution(input))
    }
}
