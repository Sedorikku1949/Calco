use std::f64::consts::PI;

use pest::{ self, Parser, iterators::Pair };

use crate::ast::{ Node, Operator };

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;

pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = CalcParser::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Expr => {
            let inner = &mut pair.clone().into_inner();
            if let Some(next) = inner.next() { build_ast_from_expr(next) }
            else {
                match pair.as_span().as_str() {
                    "pi" => Node::Number(PI),
                    _ => Node::Number(0.0)
                }
            }
        },
        Rule::UnaryExpr => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let child = pair.next().unwrap();
            let child = build_ast_from_term(child);
            parse_unary_expr(op, child)
        }
        Rule::BinaryExpr => {
            {
                let pair: Vec<Pair<Rule>> = pair.clone().into_inner().collect();
                if pair.len() < 3 {
                    // forme A(B)
                    // TODO lshpair peut être "log", "tan" etc..., il faut donc vérifier si c'est une fonction
                    // et si c'est une fonction, l'opérateur n'est pas *, mais on appel la fonction
                    // il suffit de faire un match de `lhspair`
                    // le cas `_` sera donc le code ci-dessous
                    let lhspair = pair.get(0).unwrap().to_owned();
                    let lhs = build_ast_from_term(lhspair);
                    let rhspair = pair.get(1).unwrap().to_owned();
                    let rhs = build_ast_from_term(rhspair);
                    let retval = parse_binary_expr(Operator::Multiplication, lhs, rhs);

                    return retval
                }
            }
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let mut lhs = build_ast_from_term(lhspair);
            let mut op = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            let mut rhs = build_ast_from_term(rhspair);
            let mut retval = parse_binary_expr(parse_op(op), lhs, rhs);
            loop {
                let pair_buf = pair.next();
                if pair_buf != None {
                    op = pair_buf.unwrap();
                    lhs = retval;
                    rhs = build_ast_from_term(pair.next().unwrap());
                    retval = parse_binary_expr(parse_op(op), lhs, rhs);
                } else {
                    return retval;
                }
            }
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Number => {
            let istr = pair.as_str();
            let (sign, istr): (f64, &str) = match &istr[..1] {
                "-" => (-1.0, &istr[1..]),
                _ => (1.0, istr),
            };
            let int: f64 = istr.parse().unwrap();
            Node::Number(sign * int)
        }
        Rule::Var => {
            match pair.as_str() {
                "pi" => Node::Number(PI),
                _ => Node::Number(0.0)
            }
        }
        Rule::Expr => build_ast_from_expr(pair),
        Rule::BinaryExpr => build_ast_from_expr(pair),
        unknown => panic!("Unknown term: {:?}", unknown),
    }
}

fn parse_unary_expr(pair: pest::iterators::Pair<Rule>, child: Node) -> Node {
    Node::UnaryExpr {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => unreachable!(),
        },
        child: Box::new(child),
    }
}

fn parse_binary_expr(op: Operator, lhs: Node, rhs: Node) -> Node {
    Node::BinaryExpr {
        op,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

fn parse_op(pair: pest::iterators::Pair<Rule>) -> Operator {
    match pair.as_str() {
        "+" => Operator::Plus,
        "-" => Operator::Minus,
        "*" | "x" => Operator::Multiplication,
        "/" => Operator::Division,
        _ => unreachable!(),
    }
}