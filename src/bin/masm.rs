/* use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process::exit;

use vm::inst::Inst;
use vm::stack::MikuType;

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
                "push" => acc.push(Inst::Push(MikuType::from_strs(
                    &split_line[1..split_line.len()],
                ))),
                "pop" => acc.push(Inst::Pop),
                "dupt" => acc.push(Inst::DupT(
                    split_line[1].parse().expect("EXPECTED A NUMBER AFTER DUPT"),
                )),
                "swap" => acc.push(Inst::Swap),
                "plus" => acc.push(Inst::Plus),
                "minus" => acc.push(Inst::Minus),
                "mult" => acc.push(Inst::Mult),
                "div" => acc.push(Inst::Div),
                "eq" => acc.push(Inst::Eq),
                "jmp" => acc.push(Inst::Jmp(
                    split_line[1].parse().expect("EXPECTED A NUMBER AFTER JMP"),
                )),
                "jmpz" => acc.push(Inst::JmpZ(
                    split_line[1].parse().expect("EXPECTED A NUMBER AFTER JMPZ"),
                )),
                "jmpnz" => acc.push(Inst::JmpNZ(
                    split_line[1]
                        .parse()
                        .expect("EXPECTED A NUMBER AFTER JMPNZ"),
                )),
                "dupb" => acc.push(Inst::DupB(
                    split_line[1].parse().expect("EXPECTED A NUMBER AFTER DUPB"),
                )),
                "call" => acc.push(Inst::Call(
                    split_line[1].parse().expect("EXPECTED A NUMBER AFTER CALL"),
                )),
                "ret" => acc.push(Inst::Ret),
                "retv" => acc.push(Inst::RetV),
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
 */

fn main() {
    
}