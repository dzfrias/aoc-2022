pub fn solution(input: &str) -> u32 {
    let mut all = 0;
    for line in input.lines().collect::<Vec<_>>().chunks(3) {
        let mut iter_line = line.iter();
        let (first, second, third) = (
            iter_line.next().unwrap(),
            iter_line.next().unwrap(),
            iter_line.next().unwrap(),
        );
        let vec1 = first.chars().collect::<Vec<char>>();
        let vec2 = second.chars().collect::<Vec<char>>();
        let vec3 = third.chars().collect::<Vec<char>>();

        let mut val = 0;
        for item in vec1 {
            if vec2.contains(&item) && vec3.contains(&item) {
                if item.is_uppercase() {
                    val = item as u32 - 38;
                } else {
                    val = item as u32 - 96;
                }
            }
        }
        all += val;
    }
    all
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
CrZsJsPPZsGzwwsLwLmpwMDw
";

        assert_eq!(70, solution(input))
    }
}
