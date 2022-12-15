use std::collections::HashSet;
use std::mem;

fn next_land(start: (u32, u32), places: &HashSet<(u32, u32)>, limit: u32) -> (u32, u32) {
    places
        .iter()
        .filter(|point| point.0 == start.0 && point.1 > start.1)
        .min_by(|x, y| x.1.cmp(&y.1))
        .and_then(|point| Some((point.0, point.1 - 1)))
        .unwrap_or((start.0, limit))
}

pub fn solution(input: &str) -> usize {
    let mut taken: HashSet<(u32, u32)> = HashSet::from_iter(
        input
            .lines()
            .map(|line| {
                let split = line
                    .split(" -> ")
                    .map(|point| {
                        point
                            .split(',')
                            .map(|n| n.parse::<u32>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                split
                    .windows(2)
                    .map(|points| {
                        let mut points = points.to_vec();
                        points.sort();
                        let p1 = mem::take(&mut points[0]);
                        let p2 = mem::take(&mut points[1]);
                        if p1[0] == p2[0] {
                            (p1[1]..=p2[1]).map(|n| (p1[0], n)).collect::<Vec<_>>()
                        } else {
                            (p1[0]..=p2[0]).map(|n| (n, p1[1])).collect::<Vec<_>>()
                        }
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .flatten(),
    );
    let limit = taken.iter().map(|point| point.1).max().unwrap() + 1;
    let mut sands = 0;
    loop {
        let mut falling = next_land((500, 0), &taken, limit);
        while !taken.contains(&(falling.0 + 1, falling.1 + 1))
            || !taken.contains(&(falling.0 - 1, falling.1 + 1))
        {
            if !taken.contains(&(falling.0 - 1, falling.1 + 1)) {
                falling = (falling.0 - 1, falling.1 + 1);
            } else {
                falling = (falling.0 + 1, falling.1 + 1);
            }
            falling = next_land(falling, &taken, limit);
            if falling.1 == limit {
                break;
            }
        }
        taken.insert(falling);
        sands += 1;
        if falling == (500, 0) {
            break;
        }
    }
    sands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(93, solution(input));
    }
}
