use log::info;

use bfjit::interpreter::State;
use bfjit::parser::parse;
use std::io::Read;


fn main() {
    env_logger::init();

    let mut buffer = String::new();
    let mut stdin = std::io::stdin().lock();
    let num = stdin.read_to_string(&mut buffer).expect("read from stdin");
    info!("Read {num} chars");
    let ops = parse(buffer).expect("parse ops");
    info!("Ops: {ops:?}");

    let stdout = std::io::stdout().lock();
    let mut state = State::new(ops, stdout, stdin);
    state.run();
}
