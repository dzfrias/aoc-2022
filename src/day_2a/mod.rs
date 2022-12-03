pub fn solution(input: &str) -> u32 {
    let mut all = 0u32;
    for line in input.lines() {
        all += match line {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
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
        assert_eq!(15, solution(input))
    }
}
