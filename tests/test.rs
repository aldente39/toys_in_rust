extern crate toys_in_rust;

use toys_in_rust::parser::*;
use toys_in_rust::interpreter::Interpreter;
use toys_in_rust::execute_program;

use std::collections::LinkedList;
use rstest::rstest;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let program = "define main() {61+50+9;}".to_string();
        let parsed = parse(&program);
        let mut i = Interpreter::new();
        let result = i.call_main(&parsed);
        assert_eq!(120, result);
    }
    #[test]
    fn test_factorial() {
        let program = r#"
            define main() {
                fact(5);
            }
            define fact(n) {
                if (n < 2) {
                    1;
                } else {
                    n*fact(n-1);
                }
            }
        "#.to_string();
        let parsed = parse(&program);
        let mut i = Interpreter::new();
        let result = i.call_main(&parsed);
        assert_eq!(120, result);
    }
    #[test]
    fn test_global() {
        let program = r#"
            global pi = 3;
            define main() {
                foo() * 3;
            }
            define foo() {
                pi* 2*2;
            }
        "#.to_string();
        let parsed = parse(&program);
        let mut i = Interpreter::new();
        let result = i.call_main(&parsed);
        assert_eq!(36, result);
    }
    #[test]
    fn test_while() {
        let program = r#"
            define main() {
                i = 0;
                while (i < 10) {
                    i = i + 1;
                }
                i;
            }
        "#.to_string();
        let parsed = parse(&program);
        let mut i = Interpreter::new();
        let result = i.call_main(&parsed);
        assert_eq!(10, result);
    }
    #[test]
    fn test_while2() {
        let program = r#"
            i = 0;
            while (i < 10) {
                i = i + 1;
            }
            i;
        "#.to_string();
        let parsed = parse_lines(&program);
        let i = Interpreter::new();
        let results: LinkedList<i32> = parsed.iter().map(|x| -> i32 { i.interpret(x)} ).collect();
        let result = results.into_iter().last();
        assert_eq!(10, result.unwrap());
    }
    #[test]
    fn test_println() {
        let program = r#"
            println(1+2+3+4+5);
            1+2+3+4+5;
        "#.to_string();
        let parsed = parse_lines(&program);
        let i = Interpreter::new();
        let results: LinkedList<i32> = parsed.iter().map(|x| -> i32 { i.interpret(x)} ).collect();
        let result = results.into_iter().last();
        assert_eq!(15, result.unwrap());
    }
    #[rstest(name, input, expected,
        case("test_add", r#"
            define main() { 61+50+9; }
        "#, 120),
        case("test_while", r#"
            define main() {
                i = 0;
                while (i < 10) {
                    i = i + 1;
                }
                i;
            }
        "#, 10),
        ::trace
    )]
    fn test_program(name: String, input: String, expected: i32) {
        assert_eq!(execute_program(&input), expected);
    }
}
