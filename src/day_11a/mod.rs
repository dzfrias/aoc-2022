#[derive(Debug)]
enum Number {
    Old,
    Num(u32),
}

#[derive(Debug)]
struct Operation {
    mul: bool,
    amount: Number,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    op: Operation,
    test: [u32; 3],
    inspected: u32,
}

impl Monkey {
    fn calculate_turn(&mut self) -> Vec<(u32, u32)> {
        let res = self
            .items
            .iter()
            .map(|item| {
                self.inspected += 1;
                let new_level = if self.op.mul {
                    match self.op.amount {
                        Number::Old => item * item,
                        Number::Num(n) => item * n,
                    }
                } else {
                    match self.op.amount {
                        Number::Old => item + item,
                        Number::Num(n) => item + n,
                    }
                } / 3;
                if new_level % self.test[0] == 0 {
                    (self.test[1], new_level)
                } else {
                    (self.test[2], new_level)
                }
            })
            .collect::<Vec<_>>();
        self.items.clear();
        res
    }
}

pub fn solution(input: &str) -> u32 {
    let mut monkeys = input
        .split("\n\n")
        .map(|data| {
            let lines = data.lines().collect::<Vec<&str>>();
            let items = lines[1]
                .split(' ')
                .skip(4)
                .map(|item| {
                    item.strip_suffix(',')
                        .unwrap_or(item)
                        .parse::<u32>()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            let op = {
                let strs = lines[2].split(' ').skip(6).collect::<Vec<_>>();
                let amount = if strs[1] == "old" {
                    Number::Old
                } else {
                    Number::Num(strs[1].parse().unwrap())
                };
                Operation {
                    mul: strs[0] == "*",
                    amount,
                }
            };
            let test: [u32; 3] = lines[3..=5]
                .iter()
                .map(|line| line.split(' ').last().unwrap().parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Monkey {
                items,
                op,
                test,
                inspected: 0,
            }
        })
        .collect::<Vec<_>>();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let turns = monkey.calculate_turn();
            for (target, item) in turns {
                monkeys[target as usize].items.push(item);
            }
        }
    }
    let mut inspected = monkeys
        .into_iter()
        .map(|monkey| monkey.inspected)
        .collect::<Vec<_>>();
    inspected.sort();
    inspected.last().unwrap() * inspected[inspected.len() - 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(10_605, solution(input));
    }
}
