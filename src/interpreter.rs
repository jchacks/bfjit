use std::{
    io::{Read, Write},
    time::Duration,
};

use crate::parser::Op;

struct State<O: Write, I: Read> {
    data: usize,
    instruction: usize,
    program: Vec<Op>,
    tape: [u8; 30000],
    output: O,
    input: I,
}

impl<O: Write, I: Read> State<O, I> {
    fn new(program: Vec<Op>, output: O, input: I) -> Self {
        Self {
            data: 0,
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
                // println!(
                //     "Instruction {:?} Data {:?} Op {:?}",
                //     self.instruction, self.data, op
                // );
                match op {
                    Op::Inc(i) => self.tape[self.data] += i,
                    Op::Dec(i) => self.tape[self.data] -= i,
                    Op::Right(i) => self.data += i,
                    Op::Left(i) => self.data -= i,
                    Op::Output(i) => {
                        let data = vec![self.tape[self.data]; (*i).into()];
                        write!(self.output, "{:?}", data).expect("write to output");
                    }
                    Op::Input(i) => {
                        let mut data = vec![0u8; (*i).into()];
                        self.input.read_exact(&mut data).expect("read from input");
                    }
                    _ => (),
                };
                self.instruction += 1;
                true
            }
            None => false,
        }
    }
}

pub fn interpret(ops: Vec<Op>, inp: impl Read, out: impl Write) {
    let mut state = State::new(ops, out, inp);
    let mut max_data = 0;
    while state.step() {
        max_data = max_data.max(state.data);
        // println!("{max_data:?} {:?}", &state.tape[0..max_data + 1]);
        std::thread::sleep(Duration::from_millis(100))
    }
    state.output.flush().expect("flush");
}
