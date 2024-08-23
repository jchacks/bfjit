use std::io::{Read, Write};

use log::debug;

use crate::parser::Op;

#[derive(Debug)]
enum InterpreterError {
    Underflow,
}

pub struct State<O: Write, I: Read> {
    head: usize,
    instruction: usize,
    program: Vec<Op>,
    tape: [u8; 30000],
    output: O,
    input: I,
}

impl<O: Write, I: Read> State<O, I> {
    pub fn new(program: Vec<Op>, output: O, input: I) -> Self {
        Self {
            head: 0,
            instruction: 0,
            program,
            tape: [0; 30000],
            input,
            output,
        }
    }

    fn step(&mut self) -> bool {
        match self.program.get(self.instruction) {
            Some(op) => {
                debug!(
                    "Instruction {:?} Data {:?} Op {:?}",
                    self.instruction, self.head, op
                );
                match op {
                    Op::Inc(i) => self.tape[self.head] += i,
                    Op::Dec(i) => self.tape[self.head] -= i,
                    Op::Right(i) => self.head += i,
                    Op::Left(i) => {
                        self.head = self
                            .head
                            .checked_sub(*i)
                            .ok_or(InterpreterError::Underflow)
                            .unwrap();
                    }
                    Op::Output(i) => {
                        let data = vec![self.tape[self.head]; (*i).into()];
                        for v in data {
                            write!(self.output, "{:}", v as char).expect("write to output");
                        }
                    }
                    Op::Input(i) => {
                        let mut data = vec![0u8; (*i).into()];
                        self.input.read_exact(&mut data).expect("read from input");
                    }
                    Op::JumpZero(val) => {
                        if self.tape[self.head] == 0 {
                            self.instruction = *val;
                        }
                    }
                    Op::JumpNonZero(val) => {
                        if self.tape[self.head] != 0 {
                            self.instruction = *val;
                        }
                    }
                };
                self.instruction += 1;
                true
            }
            None => false,
        }
    }

    pub fn run(&mut self) {
        let mut max_head = 0;
        while self.step() {
            max_head = max_head.max(self.head);
            debug!("{max_head:?} {:?}", &self.tape[0..max_head + 1]);
        }
        self.output.flush().expect("flush");
    }
}
