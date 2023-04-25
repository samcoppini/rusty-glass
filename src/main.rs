use glass::bytecode::BytecodeProgram;
use glass::bytecode::OpcodeIndex;
use glass::interpreter::*;
use glass::parser::*;

use std::fs::File;
use std::io::Read;

fn get_stacktrace_line(program: &BytecodeProgram, index: OpcodeIndex) -> String {
    let mut err_file = &program.files[0].1;
    for (file_index, filename) in program.files.iter() {
        if *file_index > index {
            break;
        }
        err_file = &filename;
    }

    let mut err_pos = &program.positions[0].1;
    for (pos_index, pos) in program.positions.iter() {
        if *pos_index > index {
            break;
        }
        err_pos = &pos;
    }

    format!("In file {} on line {}, column {}", err_file, err_pos.line, err_pos.col).to_owned()
}

fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        eprintln!("Usage: {} <glass-file>", args.nth(0).expect("nameless executable?"));
        std::process::exit(1);
    }

    let mut files = Vec::new();
    for arg in args.skip(1) {
        let mut file = match File::open(arg.clone()) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{:?}", err);
                std::process::exit(1);
            },
        };

        let mut file_content = Vec::new();
        file.read_to_end(&mut file_content).expect("Error reading file!");
        files.push((arg, file_content));
    }

    match parse_program(&files) {
        Ok(program) => {
            match execute_program(&program) {
                Ok(_) => (),
                Err(mut err) => {
                    eprintln!("Error: {:?}", err.error);
                    eprintln!("Traceback:");
                    err.stack_trace.reverse();
                    for trace_line in err.stack_trace {
                        eprintln!("  {}", get_stacktrace_line(&program, trace_line));
                    }
                    std::process::exit(1);
                },
            }
        },
        Err(err) => {
            eprintln!("{:?}", err);
            std::process::exit(1);
        },
    }
}
