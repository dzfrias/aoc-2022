pub fn solution(input: &str) -> u32 {
    let forest = input
        .lines()
        .map(|tree_line| {
            tree_line
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut highest = 0;
    for (line_index, tree_line) in forest.iter().enumerate() {
        for (index, tree) in tree_line.iter().enumerate() {
            let mut score = 1;
            score *= tree_line
                .iter()
                .take(index)
                .rev()
                .scan(false, |state, item| {
                    if state == &true {
                        return None;
                    }
                    if item >= &tree {
                        *state = true;
                    }
                    Some(item)
                })
                .count();
            score *= tree_line
                .iter()
                .skip(index + 1)
                .scan(false, |state, item| {
                    if state == &true {
                        return None;
                    }
                    if item >= &tree {
                        *state = true;
                    }
                    Some(item)
                })
                .count();
            let vert_trees = forest.iter().map(|trees| trees[index]).collect::<Vec<_>>();
            score *= vert_trees
                .iter()
                .take(line_index)
                .rev()
                .scan(false, |state, item| {
                    if state == &true {
                        return None;
                    }
                    if item >= &tree {
                        *state = true;
                    }
                    Some(item)
                })
                .count();
            score *= vert_trees
                .iter()
                .skip(line_index + 1)
                .scan(false, |state, item| {
                    if state == &true {
                        return None;
                    }
                    if item >= &tree {
                        *state = true;
                    }
                    Some(item)
                })
                .count();
            if score > highest {
                highest = score
            }
        }
    }
    highest as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(8, solution(input))
    }
}
