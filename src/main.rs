mod ast;
mod lexer;
mod parser;
mod eval;

use std::io::{self, Write};
use lexer::tokenize;
use parser::Parser;
use eval::eval;

fn main() -> anyhow::Result<()> {
    println!("表达式解释器（输入 Ctrl+C 退出）");

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let tokens = tokenize(&input)?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr()?;
        println!("{}", ast);
        let result = eval(&ast)?;

        println!("= {}", result);
    }

    // Ok(())
}
