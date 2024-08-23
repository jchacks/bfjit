use std::io::Read;
mod interpreter;
mod parser;

fn main() {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin().lock();
    let num = stdin.read_to_string(&mut buffer).expect("read from stdin");
    println!("Read {num} chars");
    let ops = parser::parse(buffer);
    println!("Ops: {ops:?}");

    let stdout = std::io::stdout().lock();
    interpreter::interpret(ops, stdin, stdout)
}
