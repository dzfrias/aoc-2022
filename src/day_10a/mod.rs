pub fn solution(input: &str) -> i32 {
    let mut add = 0;
    let mut x = 1;
    let mut cycle = 1;
    let mut sum = 0;
    let mut lines = input.lines().map(|line| line.split(' '));
    loop {
        if add != 0 {
            x += add;
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                sum += cycle * x;
            }
            add = 0;
            continue;
        }
        let Some(mut line) = lines.next() else {
            break;
        };
        match line.next().unwrap() {
            "noop" => {
                cycle += 1;
            }
            "addx" => {
                add = line.next().unwrap().parse::<i32>().unwrap();
                cycle += 1;
            }
            _ => panic!(),
        }
        if (cycle - 20) % 40 == 0 {
            sum += cycle * x;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(13_140, solution(input));
    }
}
