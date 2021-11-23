use crate::ast;

use pest::Parser;
#[derive(Parser)]
#[grammar = "grammer.pest"]
struct ToysParser;

use std::collections::LinkedList;

pub fn parse(contents: &String) -> ast::Program {
    let mut pairs = ToysParser::parse(Rule::program, contents).unwrap_or_else(|e| panic!("{}", e));
    let mut toplevels: LinkedList<ast::TopLevel> = LinkedList::new();
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

fn construct_toplevel_ast(pair: pest::iterators::Pair<Rule>) -> ast::TopLevel {
    match pair.as_rule() {
        Rule::topLevelDefinition => {
            construct_toplevel_ast(pair.into_inner().next().unwrap())
        },
        Rule::functionDefinition => {
            let mut tmp = pair.into_inner();
            let name = tmp.next().unwrap().as_str().to_string();
            let (args, mut body): (LinkedList<pest::iterators::Pair<Rule>>, LinkedList<pest::iterators::Pair<Rule>>) = tmp.partition(|x| x.as_rule() == Rule::identifier);
            ast::Ast::define_function(
                name,
                args.into_iter().map(|x| x.as_str().to_string()).collect(),
                construct_expression_ast(body.pop_front().unwrap())
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
            ast::Ast::if_expr(condition, then_clause, else_clause)
        },
        Rule::whileExpression => {
            let mut tmp = pair.into_inner();
            let conditon = construct_expression_ast(tmp.next().unwrap());
            let body = construct_expression_ast(tmp.next().unwrap());
            ast::Ast::while_expr(conditon, body)
        },
        Rule::blockExpression => {
            let tmp = pair.into_inner();
            let elements = tmp.map(|x| construct_expression_ast(x)).collect();
            ast::Ast::block(elements)
        },
        Rule::forInExpression => {
            let mut tmp = pair.into_inner();
            let loop_variable = tmp.next().unwrap();
            let from = tmp.next().unwrap();
            let to = tmp.next().unwrap();
            let body = tmp.next().unwrap();
            let mut block: LinkedList<ast::Expression> = LinkedList::new();
            let mut inner_block: LinkedList<ast::Expression> = LinkedList::new();
            inner_block.push_back(
                construct_expression_ast(body)
            );
            inner_block.push_back(
                ast::Ast::assignment(
                    loop_variable.as_str().to_string(),
                    ast::Ast::add(
                        ast::Ast::symbol(loop_variable.as_str().to_string()),
                        ast::Ast::integer(1)
                    )
                )
            );
            block.push_back(
                ast::Ast::assignment(
                    loop_variable.as_str().to_string(),
                    construct_expression_ast(from)
                )
            );
            block.push_back(
                ast::Ast::while_expr(
                    ast::Ast::less_or_equal(
                        ast::Ast::symbol(loop_variable.as_str().to_string()),
                        construct_expression_ast(to)
                    ),
                    ast::Ast::block(inner_block)
                )
            );
            ast::Ast::block(block)
        },
        Rule::assignment => {
            let mut tmp = pair.into_inner();
            let name = tmp.next().unwrap().as_str().to_string();
            let expr = construct_expression_ast(tmp.next().unwrap());
            ast::Ast::assignment(name, expr)
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
                        ">=" => ast::Ast::greater_or_equal(lhs, rhs),
                        "<=" => ast::Ast::less_or_equal(lhs, rhs),
                        ">" => ast::Ast::greater_than(lhs, rhs),
                        "<" => ast::Ast::less_than(lhs, rhs),
                        "==" => ast::Ast::equal_equal(lhs, rhs),
                        "!=" => ast::Ast::not_equal(lhs, rhs),
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
                            "+" => lhs = ast::Ast::add(lhs, rhs),
                            "-" => lhs = ast::Ast::subtract(lhs, rhs),
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
                            "*" => lhs = ast::Ast::multiply(lhs, rhs),
                            "/" => lhs = ast::Ast::divide(lhs, rhs),
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
            ast::Ast::integer(pair.as_str().parse().unwrap())
        },
        Rule::functionCall => {
            let mut tmp = pair.into_inner();
            let name = tmp.next().unwrap().as_str();
            let args = tmp.map(|x| construct_expression_ast(x)).collect();
            ast::Ast::call(name.to_string(), args)
        },
        Rule::labelledParameter => {
            let mut tmp = pair.into_inner();
            let name = tmp.next().unwrap().as_str().to_string();
            let parameter = construct_expression_ast(tmp.next().unwrap());
            ast::Ast::labelled_parameter(name, parameter)
        },
        Rule::labelledCall => {
            let mut tmp = pair.into_inner();
            let name = tmp.next().unwrap().as_str();
            let args = tmp.map(|x| {
                let mut y = x.into_inner();
                let name2 = y.next().unwrap().as_str().to_string();
                let parameter = construct_expression_ast(y.next().unwrap());
                *ast::Ast::labelled_parameter(name2, parameter)
            }).collect();
            ast::Ast::labelled_call(name.to_string(), args)
        },
        Rule::identifier => {
            ast::Ast::symbol(pair.as_str().to_string())
        },
        Rule::println => {
            let mut tmp = pair.into_inner();
            ast::Ast::println(construct_expression_ast(tmp.next().unwrap()))
        },
        _ => unreachable!(),
    }
}
