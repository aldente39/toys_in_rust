#![allow(dead_code)]

use crate::ast;

use std::collections::HashMap;
use std::rc::Rc;

pub struct Interpreter<'a> {
    pub variable_environment: Rc<ast::Environment>,
    pub function_environment: HashMap<String, &'a ast::FunctionDefinition>
}
impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            variable_environment: ast::Environment::new(),
            function_environment: HashMap::new(),
        }
    }
    pub fn interpret(&self, expression: &Box<dyn ast::Expression>) -> i32 {
        expression.eval(&self.variable_environment, &self.function_environment)
    }
    pub fn call_main(&mut self, program: &'a ast::Program) -> i32 {
        let toplevels = &program.definitions;
        for toplevel in toplevels.into_iter() {
            toplevel.eval(&self.variable_environment, &mut self.function_environment);
        }
        let main_function = self.function_environment.get("main");
        if main_function.is_some() {
            main_function.unwrap().body.eval(&self.variable_environment, &self.function_environment)
        } else {
            panic!("This program doesn't have main function.");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::LinkedList;

    #[test]
    fn test_10_plus_20() {
        let e: Box<dyn ast::Expression> = Box::new(ast::Ast::add(
            Box::new(ast::Ast::integer(10)),
            Box::new(ast::Ast::integer(20))
        ));
        let i = Interpreter::new();
        assert_eq!(30, i.interpret(&e));
    }
    #[test]
    fn test2() {
        let e: Box<dyn ast::Expression> = Box::new(ast::Ast::add(
            Box::new(ast::Ast::integer(10)),
            Box::new(ast::Ast::multiply(
                Box::new(ast::Ast::integer(4)),
                Box::new(ast::Ast::integer(8))
            ))
        ));
        let i = Interpreter::new();
        assert_eq!(42, i.interpret(&e));
    }
    #[test]
    fn test_assignment() {
        let i = Interpreter::new();
        let a: Box<dyn ast::Expression> = Box::new(ast::Ast::assignment("a".to_string(), Box::new(ast::Ast::integer(10))));
        i.interpret(&a);
        let b: Box<dyn ast::Expression> = Box::new(ast::Ast::assignment("b".to_string(), Box::new(ast::Ast::integer(20))));
        i.interpret(&b);
        let e: Box<dyn ast::Expression> = Box::new(ast::Ast::add(
            Box::new(ast::Ast::symbol("a".to_string())),
            Box::new(ast::Ast::symbol("b".to_string()))
        ));
        
        assert_eq!(30, i.interpret(&e));
    }
    #[test]
    fn test_factorial() {
        let mut toplevels: LinkedList<Box<dyn ast::TopLevel>> = LinkedList::new();
        let mut fact_args: LinkedList<Box<dyn ast::Expression>> = LinkedList::new();
        fact_args.push_back(Box::new(ast::Ast::integer(5)));
        let mut block_list: LinkedList<Box<dyn ast::Expression>> = LinkedList::new();
        block_list.push_back(
            Box::new(
                ast::Ast::call("fact".to_string(), fact_args)
            )
        );
        let main = ast::Ast::define_function(
            "main".to_string(),
            LinkedList::new(),
            Box::new(
                ast::Ast::block(block_list)
            )
        );
        let mut inner_fact_args: LinkedList<Box<dyn ast::Expression>> = LinkedList::new();
        inner_fact_args.push_back(
            Box::new(
                ast::Ast::subtract(
                    Box::new(ast::Ast::symbol("n".to_string())),
                    Box::new(ast::Ast::integer(1))
                )
            )
        );
        let mut block_list2: LinkedList<Box<dyn ast::Expression>> = LinkedList::new();
        block_list2.push_back(
                Box::new(ast::Ast::if_expr(
                    Box::new(ast::Ast::less_than(
                        Box::new(ast::Ast::symbol("n".to_string())),
                        Box::new(ast::Ast::integer(2))
                    )),
                    Box::new(ast::Ast::integer(1)),
                    Some(Box::new(
                        ast::Ast::multiply(
                            Box::new(ast::Ast::symbol("n".to_string())),
                            Box::new(ast::Ast::call(
                                "fact".to_string(),
                                inner_fact_args
                            ))
                        )
                    ))
                )
            )
        );
        let fact = ast::Ast::define_function(
            "fact".to_string(),
            LinkedList::from(["n".to_string()]),
            Box::new(
                ast::Ast::block(block_list2)
            )
        );
        toplevels.push_back(Box::new(main));
        toplevels.push_back(Box::new(fact));
        let mut i = Interpreter::new();
        let result = i.call_main(&ast::Program { definitions: toplevels });
        assert_eq!(120, result);
    }
}
