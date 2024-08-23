#[derive(Debug)]
pub struct ParserError(String);

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
            Self::Inc => '+',
            Self::Dec => '-',
            Self::Right => '>',
            Self::Left => '<',
            Self::Output => '.',
            Self::Input => ',',
            Self::JumpZero => '[',
            Self::JumpNonZero => ']',
            Self::Other => '#',
        }
    }

    fn new_parser_op(&self) -> Option<ParserOp> {
        match self {
            Self::Inc => Some(ParserOp::Inc(1)),
            Self::Dec => Some(ParserOp::Dec(1)),
            Self::Right => Some(ParserOp::Right(1)),
            Self::Left => Some(ParserOp::Left(1)),
            Self::Output => Some(ParserOp::Output(1)),
            Self::Input => Some(ParserOp::Input(1)),
            Self::JumpZero => Some(ParserOp::JumpZero(None)),
            Self::JumpNonZero => Some(ParserOp::JumpNonZero(None)),
            Self::Other => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ParserOp {
    Inc(u8),
    Dec(u8),
    Right(usize),
    Left(usize),
    Output(u8),
    Input(u8),
    JumpZero(Option<usize>),
    JumpNonZero(Option<usize>),
}

impl ParserOp {
    const fn token(&self) -> Token {
        match self {
            ParserOp::Inc(_) => Token::Inc,
            ParserOp::Dec(_) => Token::Dec,
            ParserOp::Right(_) => Token::Right,
            ParserOp::Left(_) => Token::Left,
            ParserOp::Output(_) => Token::Output,
            ParserOp::Input(_) => Token::Input,
            ParserOp::JumpZero(_) => Token::JumpZero,
            ParserOp::JumpNonZero(_) => Token::JumpNonZero,
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
    JumpZero(usize),
    JumpNonZero(usize),
}

impl TryFrom<ParserOp> for Op {
    type Error = ParserError;
    fn try_from(value: ParserOp) -> Result<Self, Self::Error> {
        match value {
            ParserOp::Inc(v) => Ok(Op::Inc(v)),
            ParserOp::Dec(v) => Ok(Op::Dec(v)),
            ParserOp::Right(v) => Ok(Op::Right(v)),
            ParserOp::Left(v) => Ok(Op::Left(v)),
            ParserOp::Output(v) => Ok(Op::Output(v)),
            ParserOp::Input(v) => Ok(Op::Input(v)),
            ParserOp::JumpZero(v) => v
                .ok_or(ParserError("Unknown jump to location".into()))
                .map(Op::JumpZero),
            ParserOp::JumpNonZero(v) => v
                .ok_or(ParserError("Unknown jump to location".into()))
                .map(Op::JumpNonZero),
        }
    }
}

struct Parser {
    ops: Vec<ParserOp>,
    opened_jumps: Vec<usize>,
}

impl Parser {
    fn new() -> Self {
        Self {
            ops: Vec::new(),
            opened_jumps: Vec::new(),
        }
    }
}

pub fn parse(buffer: String) -> Result<Vec<Op>, ParserError> {
    let parser = Parser::new();

    buffer
        .chars()
        .map(Token::parse)
        .fold(parser, |mut parser, e| {
            match parser.ops.last_mut() {
                Some(ParserOp::Inc(ref mut count)) if matches!(e, Token::Inc) => *count += 1,
                Some(ParserOp::Dec(ref mut count)) if matches!(e, Token::Dec) => *count += 1,
                Some(ParserOp::Right(ref mut count)) if matches!(e, Token::Right) => *count += 1,
                Some(ParserOp::Left(ref mut count)) if matches!(e, Token::Left) => *count += 1,
                Some(ParserOp::Output(ref mut count)) if matches!(e, Token::Output) => *count += 1,
                Some(ParserOp::Input(ref mut count)) if matches!(e, Token::Input) => *count += 1,
                // Initialise the Operation
                _ => {
                    let val = match e {
                        Token::Inc => ParserOp::Inc(1),
                        Token::Dec => ParserOp::Dec(1),
                        Token::Right => ParserOp::Right(1),
                        Token::Left => ParserOp::Left(1),
                        Token::Output => ParserOp::Output(1),
                        Token::Input => ParserOp::Input(1),
                        Token::JumpZero => {
                            parser.opened_jumps.push(parser.ops.len());
                            ParserOp::JumpZero(None)
                        }
                        Token::JumpNonZero => {
                            let to_location = parser.ops.len();
                            let from_location =
                                parser.opened_jumps.pop().expect("open jump locations");
                            let from = parser.ops.get_mut(from_location).expect("fetch jump from");

                            if let ParserOp::JumpZero(ref mut target) = from {
                                if target.is_none() {
                                    *target = Some(to_location);
                                } else {
                                    panic!("Targets location was not None!")
                                }
                            } else {
                                panic!("Unexpected type {from:?} found at jump from location")
                            }
                            ParserOp::JumpNonZero(Some(from_location))
                        }
                        Token::Other => return parser,
                    };
                    parser.ops.push(val)
                }
            }
            parser
        })
        .ops
        .into_iter()
        .inspect(|pop| println!("{pop:?}"))
        .collect::<Vec<ParserOp>>()
        .into_iter()
        .map(TryInto::<Op>::try_into)
        .collect()
}
