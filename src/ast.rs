// 语法树结构
use std::fmt;

use std::convert::TryFrom;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    // Bool(bool),
    UnaryOp { op: BinaryOp, expr: Box<Expr> },
    BinaryOp { left: Box<Expr>, op: BinaryOp, right: Box<Expr> }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            // Expr::Bool(b)=> write!(f, "{b}"),
            Expr::UnaryOp { op, expr } => write!(f, "({}{})", op, expr),
            Expr::BinaryOp { left, op, right } => write!(f, "({} {} {})", left, op, right),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,    // ==
    Neq,   // !=
    Gt,    // >
    Gte,   // >=
    Lt,    // <
    Lte,   // <=
    And,   // &&
    Or,    // ||
    Not,   // !
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Eq  => "==",
            BinaryOp::Neq => "!=",
            BinaryOp::Gt  => ">",
            BinaryOp::Gte => ">=",
            BinaryOp::Lt  => "<",
            BinaryOp::Lte => "<=",
            BinaryOp::And => "&&",
            BinaryOp::Or  => "||",
            BinaryOp::Not => "!",
        };
        write!(f, "{}", symbol)
    }
}


impl TryFrom<&Token> for BinaryOp {
    type Error = anyhow::Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus => Ok(BinaryOp::Add),
            Token::Minus => Ok(BinaryOp::Sub),
            Token::Star => Ok(BinaryOp::Mul),
            Token::Slash => Ok(BinaryOp::Div),
            Token::Equal => Ok(BinaryOp::Eq),
            Token::NotEqual => Ok(BinaryOp::Neq),
            Token::Greater => Ok(BinaryOp::Gt),
            Token::GreaterEqual => Ok(BinaryOp::Gte),
            Token::Less => Ok(BinaryOp::Lt),
            Token::LessEqual => Ok(BinaryOp::Lte),
            Token::And => Ok(BinaryOp::And),
            Token::Or => Ok(BinaryOp::Or),
            Token::Not => Ok(BinaryOp::Not),
            // Token::Number(n) => anyhow::bail!("错误的符号: {n}"),
            _ => Err(anyhow::bail!("未匹配的token")),
        }
    }
}