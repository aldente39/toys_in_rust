pub mod ast;
pub mod interpreter;
pub mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::LinkedList;

pub fn execute_program(program: &String) -> i32 {
    let parsed = parser::parse(&program);
    let mut i = interpreter::Interpreter::new();
    i.call_main(&parsed)
}

pub fn execute_lines(lines: &String) -> i32 {
    let parsed = parser::parse_lines(&lines);
    let i = interpreter::Interpreter::new();
    let results: LinkedList<i32> = parsed.iter().map(|x| -> i32 { i.interpret(x)} ).collect();
    results.into_iter().last().unwrap_or_else(|| 0)
}
