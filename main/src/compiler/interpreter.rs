use crate::ast::{Compile, Node, Operator};

pub struct Interpreter;

impl Compile for Interpreter {
    type Output = Result<f64, EvalError>;

    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut ret: f64 = 0.0;
        let evaluator = Eval::new();
        for node in ast {
            ret += evaluator.eval(&node)?;
        }
        Ok(ret)
    }
}

struct Eval;

pub enum EvalError {
    /// multiplication devant un nombre mais sans nombre devant (wtf ?)
    MultiplicationSyntax,
    /// division devant un nombre mais sans nombre devant (wtf x2 ?)
    DivisionSyntax,
    /// division by zero, some math has been broken
    DivisionByZero
}

impl Eval {
    pub fn new() -> Self {
        Self
    }

    pub fn eval(&self, node: &Node) -> Result<f64, EvalError> {
        match node {
            Node::Number(n) => Ok(*n),
            Node::UnaryExpr { op, child } => {
                let child = self.eval(child)?;
                match op {
                    Operator::Plus => Ok(child),
                    Operator::Minus => Ok(-child),
                    Operator::Multiplication => Err(EvalError::MultiplicationSyntax),
                    Operator::Division => Err(EvalError::DivisionSyntax),
                }
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                let lhs_ret = self.eval(lhs)?;
                let rhs_ret = self.eval(rhs)?;

                match op {
                    Operator::Plus => Ok(lhs_ret + rhs_ret),
                    Operator::Minus => Ok(lhs_ret - rhs_ret),
                    Operator::Multiplication => Ok(lhs_ret * rhs_ret),
                    Operator::Division => if rhs_ret != 0.0 { Ok(lhs_ret / rhs_ret) } else { Err(EvalError::DivisionByZero) }
                }
            }
        }
    }
}