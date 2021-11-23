use toys_in_rust::*;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("specify a file");
    }
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");

    let mut program = String::new();
    f.read_to_string(&mut program)
        .expect("something went wrong reading the file");

    execute_program(&program);
}
