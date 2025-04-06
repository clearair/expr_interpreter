use crate::{ast::{BinaryOp, Expr}, lexer::Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    depth: usize, // 用于打印缩进
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        // tokens.iter().for_each(|t| println!("{}", *t));
        Parser { tokens, pos: 0, depth: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn eat(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    fn log_enter(&mut self, name: &str) {
        println!("{:indent$}> {} depth : {}", "", name, self.depth, indent = self.depth * 2);
        self.depth += 1;
    }

    fn log_exit(&mut self, name: &str) {
        self.depth -= 1;
        println!("{:indent$}< {} depth : {}", "", name, self.depth, indent = self.depth * 2);
    }

    pub fn parse_expr(&mut self) -> anyhow::Result<Expr> {
        self.log_enter("parse_expr");
        let res = self.parse_cmd();
        self.log_exit("parse_expr");
        res
    }
    
    fn parse_cmd (&mut self) -> anyhow::Result<Expr> {
        self.log_enter("parse_cmd");
        let mut node = self.parse_term()?;
        while let Some(token) = self.current() {
            match token {
                Token::Equal | Token::NotEqual | Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual => {
                    let op = BinaryOp::try_from(token)?; // 把 Token 转成 BinaryOp
                    self.eat();
                    let right = self.parse_term()?;
                    
                    node = Expr::BinaryOp { left: Box::new(node), op, right: Box::new(right) };
                }
                _ => break,
            }
        }
        self.log_exit("parse_cmd");

        Ok(node)
    }

    fn parse_term(&mut self) -> anyhow::Result<Expr> {
        self.log_enter("parse_term");
        let mut node = self.parse_factor()?;

        while let Some(token) = self.current() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = if let Token::Plus = token { BinaryOp::Add } else { BinaryOp::Sub };
                    self.eat();
                    let right = self.parse_factor()?;
                    node = Expr::BinaryOp { left: Box::new(node), op, right: Box::new(right) }
                }
                _ => break,
            }
        }

        self.log_exit("parse_term");
        Ok(node)
    }

    fn parse_factor(&mut self) -> anyhow::Result<Expr> {
        self.log_enter("parse_factor");
        let mut node = self.parse_unary()?;
        while let Some(token) = self.current() {
            println!("{:depth$}token: {token}", "", depth = self.depth * 2);
            match token {
                Token::Star | Token::Slash => {
                    let op = if let Token::Star = token { BinaryOp::Mul } else { BinaryOp::Div };
                    self.eat();
                    let right = self.parse_unary()?;
                    node = Expr::BinaryOp { 
                        left: Box::new(node), op, right: Box::new(right) 
                    }
                }
                _ => break,
            }
        };

        self.log_exit("parse_factor");
        Ok(node)
    }

    fn parse_unary(&mut self) -> anyhow::Result<Expr> {
        self.log_enter("parse_unary");
        let res = match self.current() {
            Some(Token::Minus) | Some(Token::Plus) => {
                let op = if let Some(Token::Minus) = self.current() { BinaryOp::Sub } else { BinaryOp::Add };
                self.eat();
                Ok(Expr::UnaryOp {
                    op,
                    expr: Box::new(self.parse_primary()?),
                })
            }
            _ => self.parse_primary(),
        };
        self.log_exit("parse_unary");
        res
    }

    fn parse_primary(&mut self) -> anyhow::Result<Expr> {
        self.log_enter("parse_primary");
        let depth = self.depth;
        let res = match self.eat() {
            Some(Token::Number(n)) => {
                println!("{:indent$}=> Number({})", "", n, indent = depth * 2);
                Ok(Expr::Number(*n))
            }
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                if let Some(Token::RParen) = self.eat() {
                    Ok(expr)
                } else {
                    anyhow::bail!("括号不匹配");
                }
            }
            _ => anyhow::bail!("非法表达式"),
        };
        self.log_exit("parse_primary");
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_debug()
    {
        let tokens = vec![
            Token::Number(1f64),
            Token::Plus,
            Token::Number(2f64),
            Token::Star,
            Token::Number(3f64)
        ];
        let mut parser = Parser::new(tokens);
        assert!(parser.parse_expr().is_ok());
    }
}