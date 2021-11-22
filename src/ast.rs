mod operator;
use operator::*;

use std::collections::HashMap;
use std::collections::LinkedList;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Ast {}

impl Ast {
    pub fn add(lhs: Box<dyn Expression>, rhs:Box<dyn Expression>) -> BinaryExpression {
        BinaryExpression::new(
            Operator::ADD,
            lhs,
            rhs
        )
    }
    pub fn subtract(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> BinaryExpression {
        BinaryExpression::new(
            Operator::SUBTRACT,
            lhs,
            rhs
        )
    }
    pub fn multiply(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> BinaryExpression {
        BinaryExpression::new(
            Operator::MULTIPLY,
            lhs,
            rhs
        )
    }
    pub fn divide(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> BinaryExpression {
        BinaryExpression::new(
            Operator::DIVIDE,
            lhs,
            rhs
        )
    }
    pub fn less_than(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> BinaryExpression {
        BinaryExpression::new(
            Operator::LessThan,
            lhs,
            rhs
        )
    }
    pub fn less_or_equal(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> BinaryExpression {
        BinaryExpression::new(
            Operator::LessOrEqual,
            lhs,
            rhs
        )
    }
    pub fn greater_than(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> BinaryExpression {
        BinaryExpression::new(
            Operator::GreaterThan,
            lhs,
            rhs
        )
    }
    pub fn greater_or_equal(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> BinaryExpression {
        BinaryExpression::new(
            Operator::GreaterOrEqual,
            lhs,
            rhs
        )
    }
    pub fn equal_equal(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> BinaryExpression {
        BinaryExpression::new(
            Operator::EqualEqual,
            lhs,
            rhs
        )
    }
    pub fn not_equal(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> BinaryExpression {
        BinaryExpression::new(
            Operator::NotEqual,
            lhs,
            rhs
        )
    }
    pub fn integer(value: i32) -> IntegerLiteral {
        IntegerLiteral::new(value)
    }
    pub fn symbol(name: String) -> Identifier {
        Identifier::new(name)
    }
    pub fn assignment(name: String, expression: Box<dyn Expression>) -> Assignment {
        Assignment::new(name, expression)
    }
    pub fn block(elements: LinkedList<Box<dyn Expression>>) -> BlockExpression {
        BlockExpression::new(elements)
    }
    pub fn while_expr(condition: Box<dyn Expression>, body: Box<dyn Expression>) -> WhileExpression {
        WhileExpression::new(condition, body)
    }
    pub fn if_expr(condition: Box<dyn Expression>, then_clause: Box<dyn Expression>, else_clause: Option<Box<dyn Expression>>) -> IfExpression {
        IfExpression::new(condition, then_clause, else_clause)
    }
    pub fn define_function(name: String, args: LinkedList<String>, body: Box<dyn Expression>) -> FunctionDefinition {
        FunctionDefinition::new(name, args, body)
    }
    pub fn call(name: String, args: LinkedList<Box<dyn Expression>>) -> FunctionCall {
        FunctionCall::new(name, args)
    }
    pub fn println(body: Box<dyn Expression>) -> PrintlnExpression {
        PrintlnExpression::new(body)
    }
}

pub trait Expression {
    fn eval(
        &self,
        _variable_environment: &Rc<Environment>,
        _function_environment: &HashMap<String, &FunctionDefinition>
    ) -> i32 {
        0
    }
}

pub struct BinaryExpression {
    operator: Operator,
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
}
impl Expression for BinaryExpression {
    fn eval(&self, v: &Rc<Environment>, f: &HashMap<String, &FunctionDefinition>) -> i32 {
        let lhs: i32 = self.lhs.eval(v, f);
        let rhs: i32 = self.rhs.eval(v, f);
        match self.operator {
            Operator::ADD => lhs + rhs,
            Operator::SUBTRACT => lhs - rhs,
            Operator::MULTIPLY => lhs * rhs,
            Operator::DIVIDE => lhs / rhs,
            Operator::LessThan => if lhs < rhs { 1 } else { 0 },
            Operator::LessOrEqual => if lhs <= rhs { 1 } else { 0 },
            Operator::GreaterThan => if lhs > rhs { 1 } else { 0 },
            Operator::GreaterOrEqual => if lhs >= rhs { 1 } else { 0 },
            Operator::EqualEqual => if lhs == rhs { 1 } else { 0 },
            Operator::NotEqual => if lhs != rhs { 1 } else { 0 },
        }
    }
}
impl BinaryExpression {
    fn new(operator: Operator, lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Self {
        Self {
            operator: operator,
            lhs: lhs,
            rhs: rhs,
        }
    }
}

pub struct IntegerLiteral {
    value: i32,
}
impl Expression for IntegerLiteral {
    fn eval(&self, _v: &Rc<Environment>, _f: &HashMap<String, &FunctionDefinition>) -> i32 {
        self.value
    }
}
impl IntegerLiteral {
    fn new(value: i32) -> Self {
        Self {
            value: value,
        }
    }
}

pub struct Assignment {
    name: String,
    expression: Box<dyn Expression>,
}
impl Expression for Assignment {
    fn eval(&self, v: &Rc<Environment>, f: &HashMap<String, &FunctionDefinition>) -> i32 {
        let value = self.expression.eval(v, f);
        v.bindings.borrow_mut().insert(self.name.clone(), value);
        value
    }
}
impl Assignment {
    fn new(name: String, expression: Box<dyn Expression>) -> Self {
        Self {
            name: name,
            expression: expression,
        }
    }
}

pub struct Identifier {
    name: String,
}
impl Expression for Identifier {
    fn eval(&self, v: &Rc<Environment>, _f: &HashMap<String, &FunctionDefinition>) -> i32 {
        let bindings_opt = v.find_binding(&self.name);
        bindings_opt.unwrap().borrow().get(&self.name).unwrap().clone()
    }
}
impl Identifier {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
        }
    }
}

pub struct BlockExpression {
    elements: LinkedList<Box<dyn Expression>>,
}
impl Expression for BlockExpression {
    fn eval(&self, v: &Rc<Environment>, f: &HashMap<String, &FunctionDefinition>) -> i32 {
        let mut value = 0;
        for e in self.elements.iter() {
            value = e.eval(v, f);
        }
        value
    }
}
impl BlockExpression {
    pub fn new(elements: LinkedList<Box<dyn Expression>>) -> Self {
        Self {
            elements: elements,
        }
    }
}

pub struct WhileExpression {
    condition: Box<dyn Expression>,
    body: Box<dyn Expression>,
}
impl Expression for WhileExpression {
    fn eval(&self, v: &Rc<Environment>, f: &HashMap<String, &FunctionDefinition>) -> i32 {
        loop {
            let condition = self.condition.eval(v, f);
            if condition != 0 {
                self.body.eval(v, f);
            } else {
                break;
            }
        }
        1
    }
}
impl WhileExpression {
    pub fn new(condition: Box<dyn Expression>, body: Box<dyn Expression>) -> Self {
        Self {
            condition: condition,
            body: body,
        }
    }
}

pub struct IfExpression {
    condition: Box<dyn Expression>,
    then_clause: Box<dyn Expression>,
    else_clause: Option<Box<dyn Expression>>,
}
impl Expression for IfExpression {
    fn eval(&self, v: &Rc<Environment>, f: &HashMap<String, &FunctionDefinition>) -> i32 {
        let condition: i32 = self.condition.eval(v, f);
        if condition != 0 {
            self.then_clause.eval(v, f)
        } else {
            match &self.else_clause {
                Some(u) => u.eval(v, f),
                None => 1,
            }
        }
    }
}
impl IfExpression {
    pub fn new(condition: Box<dyn Expression>, then_clause: Box<dyn Expression>, else_clause: Option<Box<dyn Expression>>) -> Self {
        Self {
            condition: condition,
            then_clause: then_clause,
            else_clause: else_clause,
        }
    }
}

pub struct PrintlnExpression {
    body: Box<dyn Expression>
}
impl Expression for PrintlnExpression {
    fn eval(&self, v: &Rc<Environment>, f: &HashMap<String, &FunctionDefinition>) -> i32 {
        println!("{}", self.body.eval(v, f));
        0
    }
}
impl PrintlnExpression {
    pub fn new(body: Box<dyn Expression>) -> Self {
        Self {
            body: body,
        }
    }
}

pub struct Environment {
    bindings: Rc<RefCell<HashMap<String, i32>>>,
    next: Option<Rc<Environment>>, //一つ上の呼び出し元の環境
}
impl Expression for Environment {
    fn eval(&self, _v: &Rc<Environment>, _f: &HashMap<String, &FunctionDefinition>) -> i32 {
        0
    }
}
impl Environment {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            bindings: Rc::new(RefCell::new(HashMap::new())),
            next: None,
        })
    }
    pub fn find_binding(&self, name: &String) -> Option<Rc<RefCell<HashMap<String, i32>>>> {
        match self.bindings.borrow().get(name) {
            Some(_) => Some(Rc::clone(&self.bindings)),
            _ => {
                match &self.next {
                    Some(y) => {
                        let tmp = y.find_binding(name);
                        tmp
                    },
                    None => None,
                }
            }
        }
    }

}

pub trait TopLevel {
    fn eval<'a>(
        &'a self,
        _variable_environment: &Rc<Environment>,
        _function_environment: &mut HashMap<String, &'a FunctionDefinition>
    ) -> i32 {
        0
    }
}

pub struct FunctionDefinition {
    pub name: String,
    args: LinkedList<String>,
    pub body: Box<dyn Expression>,
}
impl TopLevel for FunctionDefinition {
    fn eval<'a>(&'a self, _v: &Rc<Environment>, f: &mut HashMap<String,  &'a FunctionDefinition>) -> i32 {
        f.insert(
            self.name.clone(),
            self,
        );
        0
    }
}
impl FunctionDefinition {
    pub fn new(name: String, args: LinkedList<String>, body: Box<dyn Expression>) -> Self {
        Self {
            name: name,
            args: args,
            body: body,
        }
    }
}

pub struct GlobalVariableDefinition {
    name: String,
    body: Box<dyn Expression>,
}
impl TopLevel for GlobalVariableDefinition {
    fn eval(&self, v: &Rc<Environment>, f: &mut HashMap<String, &FunctionDefinition>) -> i32 {
        v.bindings.borrow_mut().insert(
            self.name.clone(),
            self.body.eval(v, f)
        );
        0
    }
}
impl GlobalVariableDefinition {
    pub fn new(name: String, body: Box<dyn Expression>) -> Self {
        Self {
            name: name,
            body: body,
        }
    }
}

pub struct FunctionCall {
    name: String,
    args: LinkedList<Box<dyn Expression>>,
}
impl Expression for FunctionCall {
    fn eval(&self, v: &Rc<Environment>, f: &HashMap<String, &FunctionDefinition>) -> i32 {
        let definition = f.get(&self.name);
        match definition {
            Some(x) => {
                let formal_params = &x.args;
                let actual_params = &self.args;
                let body = &x.body;
                let values: LinkedList<i32> = actual_params.iter().map(|x| x.eval(v, f)).collect();
                let mut values_iter = values.into_iter();
                for formal_param_name in formal_params {
                    v.bindings.borrow_mut().insert(formal_param_name.clone(), values_iter.next().unwrap());
                }
                let result = body.eval(&new_environment(v), f);
                result
            },
            None => panic!("function not found."),
        }
    }
}
impl FunctionCall {
    pub fn new(name: String, args: LinkedList<Box<dyn Expression>>) -> Self {
        Self {
            name: name,
            args: args,
        }
    }
}

pub struct Program {
    pub definitions: LinkedList<Box<dyn TopLevel>>
}

fn new_environment(e: &Rc<Environment>) -> Rc<Environment> {
    Rc::new(Environment {
        bindings: Rc::new(RefCell::new(HashMap::new())),
        next: Some(Rc::clone(e)),
    })
}
