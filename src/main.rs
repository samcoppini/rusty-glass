use glass::interpreter::*;
use glass::parser::*;

use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <glass-file>", args[0]);
        std::process::exit(1);
    }

    let mut file = match File::open(args[1].clone()) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{:?}", err);
            std::process::exit(1);
        },
    };

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Error reading file!");

    match parse_program(&file_content) {
        Ok(program) => {
            match execute_program(&program) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("{:?}", err);
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
