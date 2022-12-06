use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(usize),

    #[token("move")]
    Move,
    #[token("from")]
    From,
    #[token("to")]
    To,

    #[error]
    #[regex(r"[ \n\t\f]+", logos::skip)]
    Error,
}

pub fn solution(input: &str) -> String {
    let mut stacks = vec![
        vec!['G', 'D', 'V', 'Z', 'J', 'S', 'B'],
        vec!['Z', 'S', 'M', 'G', 'V', 'P'],
        vec!['C', 'L', 'B', 'S', 'W', 'T', 'Q', 'F'],
        vec!['H', 'J', 'G', 'W', 'M', 'R', 'V', 'Q'],
        vec!['C', 'L', 'S', 'N', 'F', 'M', 'D'],
        vec!['R', 'G', 'C', 'D'],
        vec!['H', 'G', 'T', 'R', 'J', 'D', 'S', 'Q'],
        vec!['P', 'F', 'V'],
        vec!['D', 'R', 'S', 'T', 'J'],
    ];
    let [_, instructions]: [&str; 2] = input.split("\n\n").collect::<Vec<_>>().try_into().unwrap();
    let mut lexer = Token::lexer(instructions);
    let mut current_token = lexer.next();
    while current_token.is_some() {
        let Some(Token::Number(amount)) = lexer.next() else {
                panic!()
            };
        assert_eq!(Some(Token::From), lexer.next());
        let Some(Token::Number(from)) = lexer.next() else {
                panic!()
            };
        assert_eq!(Some(Token::To), lexer.next());
        let Some(Token::Number(to)) = lexer.next() else {
                panic!()
            };
        current_token = lexer.next();
        let mut moving = Vec::new();
        for _ in 0..amount {
            moving.push(stacks[from - 1].pop().unwrap());
        }
        stacks[to - 1].append(&mut moving.into_iter().rev().collect());
    }
    stacks
        .iter()
        .fold(String::new(), |accum, stack| {
            stack.last().unwrap().to_string() + &accum
        })
        .chars()
        .rev()
        .collect()
}
