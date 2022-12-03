pub fn solution(input: &str) -> u32 {
    let mut all = 0u32;
    for line in input.lines() {
        all += match line {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => panic!(),
        }
    }
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "A Y
B X
C Z";
        assert_eq!(12, solution(input))
    }
}
