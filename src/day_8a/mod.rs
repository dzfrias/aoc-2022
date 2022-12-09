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
    let mut visible = 0;
    for (line_index, tree_line) in forest.iter().enumerate() {
        for (index, tree) in tree_line.iter().enumerate() {
            if tree_line.iter().take(index).all(|item| item < tree)
                || tree_line.iter().skip(index + 1).all(|item| item < tree)
            {
                visible += 1;
            } else {
                let vert_trees = forest.iter().map(|trees| trees[index]).collect::<Vec<_>>();
                if vert_trees.iter().take(line_index).all(|item| item < tree)
                    || vert_trees
                        .iter()
                        .skip(line_index + 1)
                        .all(|item| item < tree)
                {
                    visible += 1;
                }
            }
        }
    }
    visible
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
        assert_eq!(21, solution(input))
    }
}
