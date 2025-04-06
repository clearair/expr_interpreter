use anyhow::Ok;

// 求值器
use crate::ast::{BinaryOp, Expr};

pub fn eval(expr: &Expr) -> anyhow::Result<f64> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::UnaryOp { op, expr } => match op {
            BinaryOp::Add | BinaryOp::Sub => {
                let mut n = eval(expr)?;
                if *op == BinaryOp::Sub {
                    n = -n;
                }
                Ok(n)
            },
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
                    if r == 0.0 {
                        anyhow::bail!("除以零错误");
                    }
                    Ok(l / r)
                },
                BinaryOp::Eq  => Ok(if l == r { 1.0 } else { 0.0 }),
                BinaryOp::Neq => Ok(if l != r { 1.0 } else { 0.0 }),
                BinaryOp::Gt  => Ok(if l >  r { 1.0 } else { 0.0 }),
                BinaryOp::Gte => Ok(if l >= r { 1.0 } else { 0.0 }),
                BinaryOp::Lt  => Ok(if l <  r { 1.0 } else { 0.0 }),
                BinaryOp::Lte => Ok(if l <= r { 1.0 } else { 0.0 }),
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
        assert_eq!(result, 42.0);
    }

    // Test for Unary Operations (e.g., -x)
    #[test]
    fn test_unary_op() {
        let expr = Expr::UnaryOp {
            op: BinaryOp::Sub,
            expr: Box::new(number_expr(5.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, -5.0);
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
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_binary_op_sub() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(5.0)),
            op: BinaryOp::Sub,
            right: Box::new(number_expr(3.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_binary_op_mul() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(4.0)),
            op: BinaryOp::Mul,
            right: Box::new(number_expr(2.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_binary_op_div() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(10.0)),
            op: BinaryOp::Div,
            right: Box::new(number_expr(2.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_binary_op_eq() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(3.0)),
            op: BinaryOp::Eq,
            right: Box::new(number_expr(3.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, 1.0); // True is represented by 1.0
    }

    #[test]
    fn test_binary_op_neq() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(3.0)),
            op: BinaryOp::Neq,
            right: Box::new(number_expr(4.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, 1.0); // True is represented by 1.0
    }

    #[test]
    fn test_binary_op_gt() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(5.0)),
            op: BinaryOp::Gt,
            right: Box::new(number_expr(3.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, 1.0); // True is represented by 1.0
    }

    #[test]
    fn test_binary_op_gte() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(5.0)),
            op: BinaryOp::Gte,
            right: Box::new(number_expr(5.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, 1.0); // True is represented by 1.0
    }

    #[test]
    fn test_binary_op_lt() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(2.0)),
            op: BinaryOp::Lt,
            right: Box::new(number_expr(3.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, 1.0); // True is represented by 1.0
    }

    #[test]
    fn test_binary_op_lte() {
        let expr = Expr::BinaryOp {
            left: Box::new(number_expr(2.0)),
            op: BinaryOp::Lte,
            right: Box::new(number_expr(2.0)),
        };
        let result = eval(&expr).unwrap();
        assert_eq!(result, 1.0); // True is represented by 1.0
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
