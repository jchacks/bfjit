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
    JumpNonZero(usize),
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
            ParserOp::JumpNonZero(v) => Ok(Op::JumpNonZero(v)),
        }
    }
}

struct Parser {
    ops: Vec<ParserOp>,
    opened_jumps: Vec<usize>,
    errors: Vec<ParserError>,
}

impl Parser {
    fn new() -> Self {
        Self {
            ops: Vec::new(),
            opened_jumps: Vec::new(),
            errors: Vec::new(),
        }
    }
}

pub fn parse(buffer: String) -> Result<Vec<Op>, ParserError> {
    let parser = Parser::new();

    let parser = buffer.chars().map(Token::parse).try_fold(
        parser,
        |mut parser, e| -> Result<Parser, ParserError> {
            match parser.ops.last_mut() {
                // TODO maybe there is a better way than repeating this statement.
                Some(ParserOp::Inc(ref mut count)) if matches!(e, Token::Inc) => *count += 1,
                Some(ParserOp::Dec(ref mut count)) if matches!(e, Token::Dec) => *count += 1,
                Some(ParserOp::Right(ref mut count)) if matches!(e, Token::Right) => *count += 1,
                Some(ParserOp::Left(ref mut count)) if matches!(e, Token::Left) => *count += 1,
                Some(ParserOp::Output(ref mut count)) if matches!(e, Token::Output) => *count += 1,
                Some(ParserOp::Input(ref mut count)) if matches!(e, Token::Input) => *count += 1,
                // Initialise the Operation
                _ => {
                    let val = match e {
                        Token::Inc => Ok(ParserOp::Inc(1)),
                        Token::Dec => Ok(ParserOp::Dec(1)),
                        Token::Right => Ok(ParserOp::Right(1)),
                        Token::Left => Ok(ParserOp::Left(1)),
                        Token::Output => Ok(ParserOp::Output(1)),
                        Token::Input => Ok(ParserOp::Input(1)),
                        Token::JumpZero => {
                            parser.opened_jumps.push(parser.ops.len());
                            Ok(ParserOp::JumpZero(None))
                        }
                        Token::JumpNonZero => {
                            let to_location = parser.ops.len();
                            let from_location = parser.opened_jumps.pop().ok_or(ParserError(
                                "No open jump locations to match with".into(),
                            ))?;
                            let from = parser
                                .ops
                                .get_mut(from_location)
                                .ok_or(ParserError(format!("No OP at location {from_location}")))?;

                            if let ParserOp::JumpZero(ref mut target) = from {
                                if target.is_none() {
                                    *target = Some(to_location);
                                    Ok(ParserOp::JumpNonZero(from_location))
                                } else {
                                    Err(ParserError(format!(
                                        "Target {target:?} location was not None.  This indicates that it was set before."
                                    )))
                                }
                            } else {
                                Err(ParserError(format!(
                                    "Unexpected type {from:?} found at jump from location"
                                )))
                            }
                        }
                        Token::Other => return Ok(parser),
                    };
                    match val {
                        Ok(v) => parser.ops.push(v),
                        Err(v) => parser.errors.push(v),
                    }
                }
            }
            Ok(parser)
        },
    );

    parser.map(|parser| {
        parser
            .ops
            .into_iter()
            .collect::<Vec<ParserOp>>()
            .into_iter()
            .map(TryInto::<Op>::try_into)
            .flat_map(|f| f.into_iter())
            .collect()
    })
}
