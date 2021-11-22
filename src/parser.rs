use crate::ast;

use pest::Parser;
#[derive(Parser)]
#[grammar = "grammer.pest"]
struct ToysParser;

use std::collections::LinkedList;

pub fn parse(contents: &String) -> ast::Program {
    let mut pairs = ToysParser::parse(Rule::program, contents).unwrap_or_else(|e| panic!("{}", e));
    let mut toplevels: LinkedList<Box<dyn ast::TopLevel>> = LinkedList::new();
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::program => {
            let tmp = pair.into_inner();
            tmp.for_each(|x| toplevels.push_back(construct_toplevel_ast(x)));
        },
        _ => unreachable!(),
    }
    ast::Program { definitions: toplevels }
}

pub fn parse_lines(contents: &String) -> LinkedList<ast::Expression> {
    let mut pairs = ToysParser::parse(Rule::lines, contents).unwrap_or_else(|e| panic!("{}", e));
    let mut lines = LinkedList::new();
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::lines => {
            for p in pair.into_inner() {
                lines.push_back(construct_expression_ast(p));
            }
        },
        _ => unreachable!(),
    }
    lines
}

fn construct_toplevel_ast(pair: pest::iterators::Pair<Rule>) -> Box<dyn ast::TopLevel> {
    match pair.as_rule() {
        Rule::topLevelDefinition => {
            construct_toplevel_ast(pair.into_inner().next().unwrap())
        },
        Rule::functionDefinition => {
            let mut tmp = pair.into_inner();
            let name = tmp.next().unwrap().as_str().to_string();
            let (args, mut body): (LinkedList<pest::iterators::Pair<Rule>>, LinkedList<pest::iterators::Pair<Rule>>) = tmp.partition(|x| x.as_rule() == Rule::identifier);
            Box::new(ast::Ast::define_function(
                name,
                args.into_iter().map(|x| x.as_str().to_string()).collect(),
                construct_expression_ast(body.pop_front().unwrap()))
            )
        },
        Rule::globalVariableDefinition => {
            let mut tmp = pair.into_inner();
            let name = tmp.next().unwrap().as_str().to_string();
            let expr = construct_expression_ast(tmp.next().unwrap());
            Box::new(ast::GlobalVariableDefinition::new(name, expr))
        },
        _ => unreachable!(),
    }
}

fn construct_expression_ast(pair: pest::iterators::Pair<Rule>) -> ast::Expression {
    match pair.as_rule() {
        Rule::line => {
            construct_expression_ast(pair.into_inner().next().unwrap())
        },
        Rule::ifExpression => {
            let mut tmp = pair.into_inner();
            let condition = construct_expression_ast(tmp.next().unwrap());
            let then_clause = construct_expression_ast(tmp.next().unwrap());
            let else_clause =  match tmp.next() {
                Some(x) => Some(construct_expression_ast(x)),
                None => None,
            };
            Box::new(ast::Ast::if_expr(condition, then_clause, else_clause))
        },
        Rule::whileExpression => {
            let mut tmp = pair.into_inner();
            let conditon = construct_expression_ast(tmp.next().unwrap());
            let body = construct_expression_ast(tmp.next().unwrap());
            Box::new(ast::Ast::while_expr(conditon, body))
        },
        Rule::blockExpression => {
            let tmp = pair.into_inner();
            let elements = tmp.map(|x| construct_expression_ast(x)).collect();
            Box::new(ast::Ast::block(elements))
        },
        Rule::assignment => {
            let mut tmp = pair.into_inner();
            let name = tmp.next().unwrap().as_str().to_string();
            let expr = construct_expression_ast(tmp.next().unwrap());
            Box::new(ast::Ast::assignment(name, expr))
        },
        Rule::expressionLine => {
            construct_expression_ast(pair.into_inner().next().unwrap())
        },
        Rule::expression => {
            construct_expression_ast(pair.into_inner().next().unwrap())
        },
        Rule::comparative => {
            let mut tmp = pair.into_inner();
            let lhs = construct_expression_ast(tmp.next().unwrap());
            match tmp.next() {
                Some(operator) => {
                    let rhs = construct_expression_ast(tmp.next().unwrap());
                    match operator.as_str() {
                        ">=" => Box::new(ast::Ast::greater_or_equal(lhs, rhs)),
                        "<=" => Box::new(ast::Ast::less_or_equal(lhs, rhs)),
                        ">" => Box::new(ast::Ast::greater_than(lhs, rhs)),
                        "<" => Box::new(ast::Ast::less_than(lhs, rhs)),
                        "==" => Box::new(ast::Ast::equal_equal(lhs, rhs)),
                        "!=" => Box::new(ast::Ast::not_equal(lhs, rhs)),
                        _ => unreachable!(),
                    }
                },
                None => lhs,
            }
        },
        Rule::additive => {
            let mut tmp = pair.into_inner();
            let mut lhs = construct_expression_ast(tmp.next().unwrap());
            loop {
                match tmp.next() {
                    Some(operator) => {
                        let rhs = construct_expression_ast(tmp.next().unwrap());
                        match operator.as_str() {
                            "+" => lhs = Box::new(ast::Ast::add(lhs, rhs)),
                            "-" => lhs = Box::new(ast::Ast::subtract(lhs, rhs)),
                            _ => unreachable!(),
                        }
                    },
                    None => break,
                }
            }
            lhs
        },
        Rule::multitive => {
            let mut tmp = pair.into_inner();
            let mut lhs: ast::Expression = construct_expression_ast(tmp.next().unwrap());
            loop{
                match tmp.next() {
                    Some(operator) => {
                        let rhs = construct_expression_ast(tmp.next().unwrap());
                        match operator.as_str() {
                            "*" => lhs = Box::new(ast::Ast::multiply(lhs, rhs)),
                            "/" => lhs = Box::new(ast::Ast::divide(lhs, rhs)),
                            _ => unreachable!(),
                        }
                    },
                    None => break,
                }
            }
            lhs
        },
        Rule::primary => {
            construct_expression_ast(pair.into_inner().next().unwrap())
        },
        Rule::integer => {
            Box::new(ast::Ast::integer(pair.as_str().parse().unwrap()))
        },
        Rule::functionCall => {
            let mut tmp = pair.into_inner();
            let name = tmp.next().unwrap().as_str();
            let args = tmp.map(|x| construct_expression_ast(x)).collect();
            Box::new(ast::Ast::call(name.to_string(), args))
        },
        Rule::identifier => {
            Box::new(ast::Ast::symbol(pair.as_str().to_string()))
        },
        Rule::println => {
            let mut tmp = pair.into_inner();
            Box::new(ast::Ast::println(construct_expression_ast(tmp.next().unwrap())))
        },
        _ => unreachable!(),
    }
}
