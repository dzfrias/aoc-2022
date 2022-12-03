pub fn solution(input: &str) -> u32 {
    let mut cals = Vec::new();
    let mut current = 0u32;
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(num) => current += num,
            Err(_) => {
                cals.push(current);
                current = 0;
            }
        }
    }
    *cals.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(24_000, solution(input))
    }
}
