mod operator;
use operator::*;

use std::collections::HashMap;
use std::collections::LinkedList;
use std::rc::Rc;
use std::cell::RefCell;

pub type Expression = Box<dyn ExpressionTrait>;
pub type TopLevel = Box<dyn TopLevelTrait>;

pub struct Ast {}

impl Ast {
    pub fn add(lhs: Expression, rhs:Expression) -> Box<BinaryExpression> {
        Box::new(BinaryExpression::new(
            Operator::ADD,
            lhs,
            rhs
        ))
    }
    pub fn subtract(lhs: Expression, rhs: Expression) -> Box<BinaryExpression> {
        Box::new(BinaryExpression::new(
            Operator::SUBTRACT,
            lhs,
            rhs
        ))
    }
    pub fn multiply(lhs: Expression, rhs: Expression) -> Box<BinaryExpression> {
        Box::new(BinaryExpression::new(
            Operator::MULTIPLY,
            lhs,
            rhs
        ))
    }
    pub fn divide(lhs: Expression, rhs: Expression) -> Box<BinaryExpression> {
        Box::new(BinaryExpression::new(
            Operator::DIVIDE,
            lhs,
            rhs
        ))
    }
    pub fn less_than(lhs: Expression, rhs: Expression) -> Box<BinaryExpression> {
        Box::new(BinaryExpression::new(
            Operator::LessThan,
            lhs,
            rhs
        ))
    }
    pub fn less_or_equal(lhs: Expression, rhs: Expression) -> Box<BinaryExpression> {
        Box::new(BinaryExpression::new(
            Operator::LessOrEqual,
            lhs,
            rhs
        ))
    }
    pub fn greater_than(lhs: Expression, rhs: Expression) -> Box<BinaryExpression> {
        Box::new(BinaryExpression::new(
            Operator::GreaterThan,
            lhs,
            rhs
        ))
    }
    pub fn greater_or_equal(lhs: Expression, rhs: Expression) -> Box<BinaryExpression> {
        Box::new(BinaryExpression::new(
            Operator::GreaterOrEqual,
            lhs,
            rhs
        ))
    }
    pub fn equal_equal(lhs: Expression, rhs: Expression) -> Box<BinaryExpression> {
        Box::new(BinaryExpression::new(
            Operator::EqualEqual,
            lhs,
            rhs
        ))
    }
    pub fn not_equal(lhs: Expression, rhs: Expression) -> Box<BinaryExpression> {
        Box::new(BinaryExpression::new(
            Operator::NotEqual,
            lhs,
            rhs
        ))
    }
    pub fn integer(value: i32) -> Box<IntegerLiteral> {
        Box::new(IntegerLiteral::new(value))
    }
    pub fn symbol(name: String) -> Box<Identifier> {
        Box::new(Identifier::new(name))
    }
    pub fn assignment(name: String, expression: Expression) -> Box<Assignment> {
        Box::new(Assignment::new(name, expression))
    }
    pub fn block(elements: LinkedList<Expression>) -> Box<BlockExpression> {
        Box::new(BlockExpression::new(elements))
    }
    pub fn while_expr(condition: Expression, body: Expression) -> Box<WhileExpression> {
        Box::new(WhileExpression::new(condition, body))
    }
    pub fn if_expr(condition: Expression, then_clause: Expression, else_clause: Option<Expression>) -> Box<IfExpression> {
        Box::new(IfExpression::new(condition, then_clause, else_clause))
    }
    pub fn define_function(name: String, args: LinkedList<String>, body: Expression) -> Box<FunctionDefinition> {
        Box::new(FunctionDefinition::new(name, args, body))
    }
    pub fn call(name: String, args: LinkedList<Expression>) -> Box<FunctionCall> {
        Box::new(FunctionCall::new(name, args))
    }
    pub fn labelled_parameter(name: String, parameter: Expression) -> Box<LabelledParameter> {
        Box::new(LabelledParameter::new(name, parameter))
    }
    pub fn labelled_call(name: String, args: LinkedList<LabelledParameter>) -> Box<LabelledCall> {
        Box::new(LabelledCall::new(name, args))
    }
    pub fn println(body: Expression) -> Box<PrintlnExpression> {
        Box::new(PrintlnExpression::new(body))
    }
}

pub trait ExpressionTrait {
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
    lhs: Expression,
    rhs: Expression,
}
impl ExpressionTrait for BinaryExpression {
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
    fn new(operator: Operator, lhs: Expression, rhs: Expression) -> Self {
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
impl ExpressionTrait for IntegerLiteral {
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
    expression: Expression,
}
impl ExpressionTrait for Assignment {
    fn eval(&self, v: &Rc<Environment>, f: &HashMap<String, &FunctionDefinition>) -> i32 {
        let value = self.expression.eval(v, f);
        v.bindings.borrow_mut().insert(self.name.clone(), value);
        value
    }
}
impl Assignment {
    fn new(name: String, expression: Expression) -> Self {
        Self {
            name: name,
            expression: expression,
        }
    }
}

pub struct Identifier {
    name: String,
}
impl ExpressionTrait for Identifier {
    fn eval(&self, v: &Rc<Environment>, _f: &HashMap<String, &FunctionDefinition>) -> i32 {
        let bindings_opt = v.find_binding(&self.name);
        bindings_opt
            .expect(&format!("undefinend variable: {}", &self.name))
            .borrow().get(&self.name).unwrap().clone()
    }
}
impl Identifier {
    fn new(name: String) -> Self {
        Self {
            name: name,
        }
    }
}

pub struct BlockExpression {
    elements: LinkedList<Expression>,
}
impl ExpressionTrait for BlockExpression {
    fn eval(&self, v: &Rc<Environment>, f: &HashMap<String, &FunctionDefinition>) -> i32 {
        let mut value = 0;
        for e in self.elements.iter() {
            value = e.eval(v, f);
        }
        value
    }
}
impl BlockExpression {
    fn new(elements: LinkedList<Expression>) -> Self {
        Self {
            elements: elements,
        }
    }
}

pub struct WhileExpression {
    condition: Expression,
    body: Expression,
}
impl ExpressionTrait for WhileExpression {
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
    fn new(condition: Expression, body: Expression) -> Self {
        Self {
            condition: condition,
            body: body,
        }
    }
}

pub struct IfExpression {
    condition: Expression,
    then_clause: Expression,
    else_clause: Option<Expression>,
}
impl ExpressionTrait for IfExpression {
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
    fn new(condition: Expression, then_clause: Expression, else_clause: Option<Expression>) -> Self {
        Self {
            condition: condition,
            then_clause: then_clause,
            else_clause: else_clause,
        }
    }
}

pub struct PrintlnExpression {
    body: Expression
}
impl ExpressionTrait for PrintlnExpression {
    fn eval(&self, v: &Rc<Environment>, f: &HashMap<String, &FunctionDefinition>) -> i32 {
        println!("{}", self.body.eval(v, f));
        0
    }
}
impl PrintlnExpression {
    fn new(body: Expression) -> Self {
        Self {
            body: body,
        }
    }
}

pub struct Environment {
    bindings: Rc<RefCell<HashMap<String, i32>>>,
    next: Option<Rc<Environment>>, //????????????????????????????????????
}
impl ExpressionTrait for Environment {
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
                        y.find_binding(name)
                    },
                    None => None,
                }
            }
        }
    }

}

pub trait TopLevelTrait {
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
    pub body: Expression,
}
impl TopLevelTrait for FunctionDefinition {
    fn eval<'a>(&'a self, _v: &Rc<Environment>, f: &mut HashMap<String,  &'a FunctionDefinition>) -> i32 {
        f.insert(
            self.name.clone(),
            self,
        );
        0
    }
}
impl FunctionDefinition {
    pub fn new(name: String, args: LinkedList<String>, body: Expression) -> Self {
        Self {
            name: name,
            args: args,
            body: body,
        }
    }
}

pub struct GlobalVariableDefinition {
    name: String,
    body: Expression,
}
impl TopLevelTrait for GlobalVariableDefinition {
    fn eval(&self, v: &Rc<Environment>, f: &mut HashMap<String, &FunctionDefinition>) -> i32 {
        v.bindings.borrow_mut().insert(
            self.name.clone(),
            self.body.eval(v, f)
        );
        0
    }
}
impl GlobalVariableDefinition {
    pub fn new(name: String, body: Expression) -> Self {
        Self {
            name: name,
            body: body,
        }
    }
}

pub struct FunctionCall {
    name: String,
    args: LinkedList<Expression>,
}
impl ExpressionTrait for FunctionCall {
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
            None => panic!("function {} not found.", self.name),
        }
    }
}
impl FunctionCall {
    fn new(name: String, args: LinkedList<Expression>) -> Self {
        Self {
            name: name,
            args: args,
        }
    }
}

pub struct LabelledParameter {
    name: String,
    parameter: Expression,
}
impl ExpressionTrait for LabelledParameter {}
impl LabelledParameter {
    fn new(name: String, parameter: Expression) -> Self {
        Self {
            name: name,
            parameter: parameter,
        }
    }    
}

pub struct LabelledCall {
    name: String,
    args: LinkedList<LabelledParameter>,
}
impl ExpressionTrait for LabelledCall {
    fn eval(&self, v: &Rc<Environment>, f: &HashMap<String, &FunctionDefinition>) -> i32 {
        let definition = f.get(&self.name);
        match definition {
            Some(x) => {
                let labels = &self.args;
                let mut mapping: HashMap<&String, &Expression> = HashMap::new();
                labels.iter().for_each(|x| { mapping.insert(&x.name, &x.parameter); });

                let formal_params = &x.args;
                let mut actual_params = LinkedList::new();
                formal_params.iter().for_each(|x| actual_params.push_back(mapping.get(x).unwrap()));
                let body = &x.body;
                let values: LinkedList<i32> = actual_params.iter().map(|x| x.eval(v, f)).collect();
                let mut values_iter = values.into_iter();
                for formal_param_name in formal_params {
                    v.bindings.borrow_mut().insert(formal_param_name.clone(), values_iter.next().unwrap());
                }
                let result = body.eval(&new_environment(v), f);
                result
            },
            None => panic!("function {} not found.", self.name),
        }
    }
}
impl LabelledCall {
    fn new(name: String, args: LinkedList<LabelledParameter>) -> Self {
        Self {
            name: name,
            args: args,
        }
    }
}

pub struct Program {
    pub definitions: LinkedList<TopLevel>
}

fn new_environment(e: &Rc<Environment>) -> Rc<Environment> {
    Rc::new(Environment {
        bindings: Rc::new(RefCell::new(HashMap::new())),
        next: Some(Rc::clone(e)),
    })
}
