use std::cmp::Ordering;
use std::iter::Peekable;

use logos::{Lexer, Logos};

#[derive(Debug, Logos, PartialEq, Eq)]
enum Token {
    #[token("[")]
    Lbracket,
    #[token("]")]
    Rbracket,
    #[token(",")]
    Comma,

    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(u32),

    #[error]
    Error,
}

#[derive(Debug, PartialEq, Eq, Ord)]
enum Item {
    List(Vec<Item>),
    Num(u32),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Num(n1), Self::Num(n2)) => n1.partial_cmp(n2),
            (Self::List(list1), Self::List(list2)) => {
                for (item1, item2) in list1.iter().zip(list2) {
                    if item1.partial_cmp(item2) == Some(Ordering::Equal) {
                        continue;
                    }
                    return item1.partial_cmp(item2);
                }
                list1.len().partial_cmp(&list2.len())
            }
            (Self::List(_), Self::Num(n2)) => self.partial_cmp(&Self::List(vec![Self::Num(*n2)])),
            (Self::Num(n1), Self::List(_)) => Self::List(vec![Self::Num(*n1)]).partial_cmp(other),
        }
    }
}

fn parse_list(lexer: &mut Peekable<Lexer<Token>>) -> Vec<Item> {
    let mut list = Vec::new();
    let mut token = lexer.next().unwrap();
    if token == Token::Rbracket {
        return Vec::new();
    }
    {
        let result = match token {
            Token::Number(num) => Item::Num(num),
            Token::Lbracket => Item::List(parse_list(lexer)),
            _ => panic!(),
        };
        list.push(result);
    }
    token = lexer.next().unwrap();
    while token == Token::Comma {
        token = lexer.next().unwrap();
        let result = match token {
            Token::Number(num) => Item::Num(num),
            Token::Lbracket => Item::List(parse_list(lexer)),
            _ => panic!(),
        };
        list.push(result);
        token = lexer.next().unwrap();
    }
    list
}

pub fn solution(input: &str) -> usize {
    let input = "[[2]]\n[[6]]\n".to_owned() + input;
    let mut packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut lexer = Token::lexer(line).peekable();
            assert_eq!(Token::Lbracket, lexer.next().unwrap());
            Item::List(parse_list(&mut lexer))
        })
        .collect::<Vec<_>>();
    packets.sort();
    let pos1 = packets
        .iter()
        .position(|packet| packet == &Item::List(vec![Item::List(vec![Item::Num(2)])]))
        .unwrap()
        + 1;
    let pos2 = packets
        .iter()
        .position(|packet| packet == &Item::List(vec![Item::List(vec![Item::Num(6)])]))
        .unwrap()
        + 1;
    pos1 * pos2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(140, solution(input));
    }
}
