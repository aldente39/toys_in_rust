extern crate toys_in_rust;

use toys_in_rust::parser::*;
use toys_in_rust::interpreter::Interpreter;
use toys_in_rust::execute_program;
use toys_in_rust::execute_lines;

use std::collections::LinkedList;
use rstest::rstest;

#[cfg(test)]
mod tests {
    use super::*;

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
        case("test_while_lines", r#"
            i = 0;
            while (i < 10) {
                i = i + 1;
            }
            i;
        "#, 10),
        case("test_assignment", r#"
            a = 1 + 2;
            a = a + 3;
            a = a + a;
        "#, 12),
        case("test_if1", r#"
            if(1 >= 2) {
                a = 1;
            } else {
                a = 0;
            }
        "#, 0),
        case("test_if2", r#"
            if(1 <= 2) {
                a = 1;
            } else {
                a = 0;
            }
        "#, 1),
        case("test_for_in", r#"
            for (i in 1 to 10) {
                i = i + 1;
            }
            i;
        "#, 11),
        case("test_for_in2", r#"
            x = 20;
            for (i in 20-15 to 2*5*2) x = x + 1;
            x;
        "#, 36),
        ::trace
    )]
    fn test_lines(name: String, input: String, expected: i32) {
        assert_eq!(execute_lines(&input), expected);
    }
    #[rstest(name, input, expected,
        case("test_add", r#"
            define main() { 61+50+9; }
        "#, 120),
        case("test_factorial", r#"
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
        case("test_global", r#"
            global pi = 3;
            define main() {
                foo() * 3;
            }
            define foo() {
                pi* 2*2;
            }
        "#, 36),
        case("test_multiply", r#"
            define main() {
                ((3+4)*5+1)/2*2*3-5-4;
            }
        "#, 99),
        case("test_define_function", r#"
            define main() {
                v = add2(v);
            }
            global v = 2;
            define add2(x) {
                x + 2;
            }
        "#, 4),
        case("test_factorial2", r#"
            define factorial(n) {
                if(n < 2) {
                    1;
                } else {
                    n * factorial(n - 1);
                }
            }
            global n = 0;
            define main() {
                n = factorial(6);
                println(n);
                n;
            }
        "#, 720),
        case("test_labelled", r#"
            define power(n) {
                n * n;
            }
            define main() {
                power[n = 6];
            }
        "#, 36),
        case("test_labelled2", r#"
            define mul(n, m) {
                n * m;
            }
            define main() {
                mul[n = 6, m = 21];
            }
        "#, 126),
        ::trace
    )]
    fn test_program(name: String, input: String, expected: i32) {
        assert_eq!(execute_program(&input), expected);
    }
}
