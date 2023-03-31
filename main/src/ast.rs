use std::{fmt, error::Error};

use crate::parser;

/// Declare all possible operators for a number
/// Plus is a addition
/// Minus is a subtraction
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Multiplication => write!(f, "*"),
            Operator::Division => write!(f, "/"),
        }
    }
}

/// Create a node based on the princip of Binary Trees
/// Each node can be a Int (on 64 bits), a unary expression (with an operator and a child aka Node) and a binary expression (who take a operator and two Node (lhs & rhs))
/// Doesn't support floating values
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// Basic number, the leaf of a branch
    /// Always a floating point
    Number(f64),
    /// Same as a leaf, but is a branch that contain only one leaf (Operator)
    UnaryExpr {
        op: Operator,
        child: Box<Node>
    },
    /// A child node, aka a branch with two branchs
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>
        
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::Number(n) => write!(f, "{n}"),
            Node::UnaryExpr { op, child } => write!(f, "{op}{child}"),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "{lhs} {op} {rhs}"),
        }
    }
}


//
//
//  COMPILE TRAIT
//
//

pub trait Compile {
    type Output;

    fn from_ast(ast: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Result<Self::Output, Box<dyn Error>> {
        match parser::parse(source) {
            Ok(ast) => Ok(Self::from_ast(ast)),
            Err(err) => Err(Box::new(err))
        }
    }
}