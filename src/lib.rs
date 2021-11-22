pub mod ast;
pub mod interpreter;
pub mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub fn execute_program(program: &String) -> i32 {
    let parsed = parser::parse(&program);
    let mut i = interpreter::Interpreter::new();
    i.call_main(&parsed)
}
