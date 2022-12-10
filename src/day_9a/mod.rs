use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    let mut head: (i32, i32) = (0, 0);
    let mut tail = (0, 0);
    let mut states = HashSet::<(i32, i32)>::from_iter(vec![tail]);
    for (dir, amount) in input.lines().map(|line| {
        let mut split = line.split(' ');
        (
            split.next().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        )
    }) {
        for _ in 0..amount {
            match dir {
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                _ => panic!(),
            }
            let distance = (head.0 - tail.0, head.1 - tail.1);
            if distance.0.abs() > 1 {
                tail.0 += distance.0 - distance.0.signum();
                tail.1 += distance.1;
            }
            if distance.1.abs() > 1 {
                tail.0 += distance.0;
                tail.1 += distance.1 - distance.1.signum();
            }
            states.insert(tail);
        }
    }
    states.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, solution(input));
    }
}
