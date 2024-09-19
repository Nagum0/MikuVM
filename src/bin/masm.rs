use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process::exit;
use vm::{Inst, StackEntry};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Incorrect usage\nUsage: masm <source.masm> <output.mm>");
        exit(1);
    }

    let program = parse_file(&args[1]);
    write_bytes(&args[2], &program);
}

fn parse_file(path: &str) -> Vec<Inst> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            if line == "" {
                return acc;
            }

            let split_line = line.split(' ').collect::<Vec<&str>>();

            match split_line[0] {
                "push" => acc.push(Inst::Push(StackEntry::from_strs(
                    &split_line[1..split_line.len()],
                ))),
                "pop" => acc.push(Inst::Pop),
                "print_char_dbg" => acc.push(Inst::PrintCharDbg),
                _ => panic!("UNKNOWN INSTRUCTION: {}", split_line[0]),
            }

            acc
        })
}

fn write_bytes(path: &str, program: &[Inst]) {
    let mut file = File::create(path).unwrap();

    for inst in program {
        let mut inst_bytes = inst.to_bytes();
        inst_bytes.insert(0, inst_bytes.len() as u8);
        file.write(&inst_bytes).unwrap();
    }
}
