// 词法分析

use std::fmt::{write, Display};

use anyhow::{Context, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Equal,      // ==
    NotEqual,   // !=
    Greater,    // >
    GreaterEqual, // >=
    Less,
    LessEqual,
    And,
    Or,
    Not
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::Greater => write!(f, ">"),
            Token::GreaterEqual => write!(f, ">="),
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "<="),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Not => write!(f, "!"),
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0' ..='9' | '.' => {
                let mut number = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_numeric() || d == '.' {
                        number.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Number(number.parse().with_context(|| format!("数字解析失败: {number}"))?));
            }
            '+' => { tokens.push(Token::Plus); chars.next(); }
            '-' => { tokens.push(Token::Minus); chars.next(); }
            '*' => { tokens.push(Token::Star); chars.next(); }
            '/' => { tokens.push(Token::Slash); chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            '&' | '|' => {
                chars.next();
                match (ch, chars.peek()) {
                    ('&', Some('&')) => {
                        chars.next();
                        tokens.push(Token::And);
                    }
                    ('|', Some('|')) => {
                        chars.next();
                        tokens.push(Token::Or);
                    }
                    (_, Some(c)) => anyhow::bail!("错误的bool运算符 {ch}{c}"),
                    _ => anyhow::bail!("错误的bool运算符 {ch}")
                }
            }
            '=' | '!' => {
                chars.next();
                match (ch, chars.peek()) {
                    ('=', Some('=')) => {
                        chars.next();
                        tokens.push(Token::Equal);
                    }
                    ('!', Some('=')) => {
                        chars.next();
                        tokens.push(Token::NotEqual);
                    }
                    ('=', _) => {
                        anyhow::bail!("单个 '=' 是无效的");
                    }
                    ('!', _) => {
                        tokens.push(Token::Not); // 只有是表示 非操作符号
                    }
                    _ => {
                        anyhow::bail!("我认为永远不会执行到这里，但是编译器觉得有问题")
                    }
                }
            }
            '<' | '>' => {
                chars.next();
                match (ch, chars.peek()) {
                    ('<', Some('=')) => {
                        chars.next();
                        tokens.push(Token::LessEqual);
                    }
                    ('>', Some('=')) => {
                        chars.next();
                        tokens.push(Token::GreaterEqual);
                    }
                    ('<', _) => {
                        tokens.push(Token::Less);
                    }
                    ('>', _) => {
                        tokens.push(Token::Greater);
                    }
                    _ => {
                        anyhow::bail!("我认为永远不会执行到这里，但是编译器觉得有问题")
                    }
                }
            }
            ' ' | '\t' | '\n' => { chars.next(); },
            _ => { 
                println!("{}", ch.to_ascii_lowercase() as u8);
                anyhow::bail!("错误的字符: {ch}") 
            }
        }
    }
    Ok(tokens)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_operators() {
        let input = "+ - * / ( )";
        let tokens = tokenize(input).unwrap();
        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::Star,
            Token::Slash,
            Token::LParen,
            Token::RParen,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comparison_operators() {
        let input = "== != > >= < <=";
        let tokens = tokenize(input).unwrap();
        let expected = vec![
            Token::Equal,
            Token::NotEqual,
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_number_parsing() {
        let input = "123 4.56";
        let tokens = tokenize(input).unwrap();
        let expected = vec![
            Token::Number(123.0),
            Token::Number(4.56),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_mixed_expression() {
        let input = "1 + 2 * (3 - 4) / 5 == 6";
        let tokens = tokenize(input).unwrap();
        let expected = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::Star,
            Token::LParen,
            Token::Number(3.0),
            Token::Minus,
            Token::Number(4.0),
            Token::RParen,
            Token::Slash,
            Token::Number(5.0),
            Token::Equal,
            Token::Number(6.0),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_invalid_character() {
        let input = "1 + $";
        let result = tokenize(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_less_character() {
        let input = "1 < 2+1";
        let tokens = tokenize(input);
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert_eq!(tokens, vec![
            Token::Number(1.0),
            Token::Less,
            Token::Number(2.0),
            Token::Plus,
            Token::Number(1.0),
        ]);
    }

    #[test]
    fn test_bool_not_character() {
        let input = "1+2!3+1";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::Not,
            Token::Number(3.0),
            Token::Plus,
            Token::Number(1.0)
        ]);
    }

    #[test]
    fn test_bool_and_or_character() {
        let input = "&&!||!=!!=";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, vec![
            Token::And,
            Token::Not,
            Token::Or,
            Token::NotEqual,
            Token::Not,
            Token::NotEqual,
        ]);
    }
}
