use std::{env, fs, process::exit};

use vm::inst::Inst;
use vm::miku::Miku;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Incorrect usage\nUsage: <input.mm>");
        exit(1);
    }

    let mut miku = Miku::new();
    let bytes = read_bytes(&args[1]);
    let program = create_program(split_bytes_by_size(&bytes));
    miku.program = program;
    miku.run_program();
}

fn read_bytes(path: &str) -> Vec<u8> {
    fs::read(path).unwrap()
}

fn split_bytes_by_size(bytes: &[u8]) -> Vec<&[u8]> {
    let mut result = Vec::new();
    let mut start: usize = 0;
    let mut end: usize = bytes[start] as usize + 1;

    while start < bytes.len() && end < bytes.len() {
        end = start + bytes[start] as usize + 1;
        result.push(&bytes[start + 1..end]);
        start = end;
    }

    result
}

fn create_program(split_bytes: Vec<&[u8]>) -> Vec<Inst> {
    split_bytes.iter().fold(Vec::new(), |mut acc, bytes| {
        acc.push(Inst::from_bytes(&bytes));
        acc
    })
}
