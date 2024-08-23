#[derive(Debug, PartialEq, Eq)]
enum Token {
    Inc,
    Dec,
    Right,
    Left,
    Output,
    Input,
    JumpZero,
    JumpNonZero,
    Other,
}

impl Token {
    fn parse(c: char) -> Self {
        match c {
            '+' => Token::Inc,
            '-' => Token::Dec,
            '>' => Token::Right,
            '<' => Token::Left,
            '.' => Token::Output,
            ',' => Token::Input,
            '[' => Token::JumpZero,
            ']' => Token::JumpNonZero,
            _ => Token::Other,
        }
    }

    fn char(&self) -> char {
        match self {
            Token::Inc => '+',
            Token::Dec => '-',
            Token::Right => '>',
            Token::Left => '<',
            Token::Output => '.',
            Token::Input => ',',
            Token::JumpZero => '[',
            Token::JumpNonZero => ']',
            Token::Other => '#',
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Inc(u8),
    Dec(u8),
    Right(usize),
    Left(usize),
    Output(u8),
    Input(u8),
    JumpZero,
    JumpNonZero,
}

impl Op {
    const fn token(&self) -> Token {
        match self {
            Op::Inc(_) => Token::Inc,
            Op::Dec(_) => Token::Dec,
            Op::Right(_) => Token::Right,
            Op::Left(_) => Token::Left,
            Op::Output(_) => Token::Output,
            Op::Input(_) => Token::Input,
            Op::JumpZero => Token::JumpZero,
            Op::JumpNonZero => Token::JumpNonZero,
        }
    }
}

pub fn parse(buffer: String) -> Vec<Op> {
    buffer
        .chars()
        .map(Token::parse)
        .fold(Vec::new(), |mut acc, e| {
            match acc.last_mut() {
                Some(Op::Inc(ref mut count)) if matches!(e, Token::Inc) => *count += 1,
                Some(Op::Dec(ref mut count)) if matches!(e, Token::Dec) => *count += 1,
                Some(Op::Right(ref mut count)) if matches!(e, Token::Right) => *count += 1,
                Some(Op::Left(ref mut count)) if matches!(e, Token::Left) => *count += 1,
                Some(Op::Output(ref mut count)) if matches!(e, Token::Output) => *count += 1,
                Some(Op::Input(ref mut count)) if matches!(e, Token::Input) => *count += 1,
                _ => acc.push(match e {
                    Token::Inc => Op::Inc(1),
                    Token::Dec => Op::Dec(1),
                    Token::Right => Op::Right(1),
                    Token::Left => Op::Left(1),
                    Token::Output => Op::Output(1),
                    Token::Input => Op::Input(1),
                    Token::JumpZero => Op::JumpZero,
                    Token::JumpNonZero => Op::JumpNonZero,
                    Token::Other => return acc,
                }),
            }
            acc
        })    
}
