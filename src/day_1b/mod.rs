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
    cals.sort();
    cals.windows(3).last().unwrap().iter().sum::<u32>()
}
