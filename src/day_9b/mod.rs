use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    let mut ropes = [(0, 0); 10];
    let mut states = HashSet::<(i32, i32)>::from_iter(vec![(0, 0)]);
    for (dir, amount) in input.lines().map(|line| {
        let mut split = line.split(' ');
        (
            split.next().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        )
    }) {
        for _ in 0..amount {
            match dir {
                "U" => ropes[0].1 += 1,
                "D" => ropes[0].1 -= 1,
                "L" => ropes[0].0 -= 1,
                "R" => ropes[0].0 += 1,
                _ => panic!(),
            }
            for i in 0..9 {
                let rope1 = ropes[i];
                let mut rope2 = &mut ropes[i + 1];
                let distance: (i32, i32) = (rope1.0 - rope2.0, rope1.1 - rope2.1);
                if distance.0.abs() > 1 && distance.1.abs() > 1 {
                    rope2.0 += distance.0 - distance.0.signum();
                    rope2.1 += distance.1 - distance.1.signum();
                } else if distance.0.abs() > 1 {
                    rope2.0 += distance.0 - distance.0.signum();
                    rope2.1 += distance.1;
                } else if distance.1.abs() > 1 {
                    rope2.0 += distance.0;
                    rope2.1 += distance.1 - distance.1.signum();
                }
            }
            states.insert(ropes[9]);
        }
    }
    states.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(36, solution(input));
    }
}
