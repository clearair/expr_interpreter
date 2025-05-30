use std::{fmt::Display, ops::{Add, Div, Mul, Sub}};

// 求值器
use crate::ast::{BinaryOp, Expr};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    Bool(bool),
}

impl Value {
    fn and(&self, right: &Value) -> anyhow::Result<bool> {
        match (self, right) {
            (Value::Number(a), Value::Number(b)) => {
                if *a > 0.0 && *b > 0.0 {
                    return Ok(true);
                }

                Ok(false)
            },
            (Value::Number(a), Value::Bool(b)) => Ok(*a > 0.0 && *b),
            (Value::Bool(a), Value::Bool(b)) => Ok(*a && *b),
            (Value::Bool(a), Value::Number(b)) => Ok(*a && *b > 0.0),
        }
    }

    fn or(&self, right: &Value) -> anyhow::Result<bool> {
        match (self, right) {
            (Value::Number(a), Value::Number(b)) => {
                if *a > 0.0 || *b > 0.0 {
                    return Ok(true);
                }

                Ok(false)
            },
            (Value::Number(a), Value::Bool(b)) => Ok(*a > 0.0 || *b),
            (Value::Bool(a), Value::Bool(b)) => Ok(*a || *b),
            (Value::Bool(a), Value::Number(b)) => Ok(*a || *b > 0.0),
        }
    }

}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(n) => n,
            Value::Bool(b) => if b {1.0} else {0.0}
        }
    }
}

impl From<&Value> for f64 {
    fn from(value: &Value) -> Self {
        match value {
            Value::Number(n) => *n,
            Value::Bool(b) => if *b {1.0} else {0.0}
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Value::Number(f64::from(self) + f64::from(rhs))
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        Value::Number(f64::from(self) - f64::from(rhs))
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        Value::Number(f64::from(self) * f64::from(rhs))
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        Value::Number(f64::from(self) / f64::from(rhs))
    }
}

pub fn eval(expr: &Expr) -> anyhow::Result<Value> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::UnaryOp { op, expr } => match op {
            BinaryOp::Add | BinaryOp::Sub => {
                let mut n = eval(expr)?;
                
                match n {
                    Value::Number(num) => {
                        if *op == BinaryOp::Sub {
                            n = Value::Number(-num);
                        }
                        
                    }
                    Value::Bool(b) => {
                        if *op == BinaryOp::Sub {
                            n = Value::Bool(!b);
                        }
                    }
                    // _ => anyhow::bail!("错误 bool 值无法数学运算"),
                }

                Ok(n)
            }
            BinaryOp::Not => {
                // let mut n = eval(expr)?;
                match eval(expr)? {
                    Value::Number(num) => {
                        Ok(Value::Bool(num == 0.0))
                    }
                    Value::Bool(b) => {
                        Ok(Value::Bool(!b))
                    }
                }
            }
            _ => anyhow::bail!("不支持的单目运算符"),
        }
        Expr::BinaryOp { left, op, right } => {
            let l = eval(left)?;
            let r = eval(right)?;

            match op {
                BinaryOp::Add => Ok(l + r),
                BinaryOp::Sub => Ok(l - r),
                BinaryOp::Mul => Ok(l * r),
                BinaryOp::Div => {
                    if r == Value::Number(0.0) || r == Value::Bool(false) {
                        anyhow::bail!("除以零错误");
                    }
                    Ok(l / r)
                },
                BinaryOp::Eq  => Ok(if l == r { Value::Bool(true) } else { Value::Bool(false)}),
                BinaryOp::Neq => Ok(if l != r { Value::Bool(true) } else { Value::Bool(false)}),
                BinaryOp::Gt  => Ok(if l >  r { Value::Bool(true) } else { Value::Bool(false)}),
                BinaryOp::Gte => Ok(if l >= r { Value::Bool(true) } else { Value::Bool(false)}),
                BinaryOp::Lt  => Ok(if l <  r { Value::Bool(true) } else { Value::Bool(false)}),
                BinaryOp::Lte => Ok(if l <= r { Value::Bool(true) } else { Value::Bool(false)}),
                BinaryOp::And => Ok(if l.and(&r)? {Value::Bool(true)} else {Value::Bool(false)}),
                BinaryOp::Or => Ok(if l.or(&r)? {Value::Bool(true)} else {Value::Bool(false)}),
                _ => anyhow::bail!("不支持的双目运算符"),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOp, Expr};

    // Helper function to simplify creating BinaryOp::Number expressions
    fn number_expr(n: f64) -> Expr {
        Expr::Number(n)
    }

    // Test for simple numbers
    #[test]
    fn test_number() {
        let expr = number_expr(42.0);
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    // Test for Unary Operations (e.g., -x)
    #[test]
    fn test_unary_op() {
        let expr = Expr::UnaryOp {
            op: BinaryOp::Sub,
            expr: Box::new(number_expr(5.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    // Test for Binary Operations (e.g., 2 + 3)
    #[test]
    fn test_binary_op_add() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(2.0)),
            op: BinaryOp::Add,
            right: Box::new(number_expr(3.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_binary_op_sub() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(5.0)),
            op: BinaryOp::Sub,
            right: Box::new(number_expr(3.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn test_binary_op_mul() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(4.0)),
            op: BinaryOp::Mul,
            right: Box::new(number_expr(2.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    #[test]
    fn test_binary_op_div() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(10.0)),
            op: BinaryOp::Div,
            right: Box::new(number_expr(2.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_binary_op_eq() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(3.0)),
            op: BinaryOp::Eq,
            right: Box::new(number_expr(3.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Bool(true)); // True is represented by 1.0
    }

    #[test]
    fn test_binary_op_neq() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(3.0)),
            op: BinaryOp::Neq,
            right: Box::new(number_expr(4.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_binary_op_gt() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(5.0)),
            op: BinaryOp::Gt,
            right: Box::new(number_expr(3.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Bool(true)); // True is represented by 1.0
    }

    #[test]
    fn test_binary_op_gte() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(5.0)),
            op: BinaryOp::Gte,
            right: Box::new(number_expr(5.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Bool(true)); // True is represented by 1.0
    }

    #[test]
    fn test_binary_op_lt() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(2.0)),
            op: BinaryOp::Lt,
            right: Box::new(number_expr(3.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Bool(true)); // True is represented by 1.0
    }

    #[test]
    fn test_binary_op_lte() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(2.0)),
            op: BinaryOp::Lte,
            right: Box::new(number_expr(2.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, Value::Bool(true)); // True is represented by 1.0
    }

    // Test division by zero, should return an error
    #[test]
    fn test_divide_by_zero() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(10.0)),
            op: BinaryOp::Div,
            right: Box::new(number_expr(0.0)),
        };
        let result = eval(&expr);
        assert!(result.is_err());
    }
}
