use core::panic;

use crate::tokenizer::*;

pub struct Parser {
    tok: Tokenizer
}

pub enum AstNode {
    Assign,
    Binary,
    Unary,
    Call,
    Block,
    Literal,
    Declaration,
    If,
    While,
    When,
    Jump, // break, return, continue
    Typeof,
    // Import,
    // Export,
}

impl Parser {
    pub fn new(tok: Tokenizer) -> Parser {
        Parser {
            tok
        }
    }

    // parse until EOF
    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut program: Vec<AstNode> = Vec::new();
        while self.tok.peek().is_some() {
            program.push(self.parse_exp());
        };

        program
    }

    fn parse_exp(&mut self) -> AstNode {
        let exp = self.parse_exp_atom();

        panic!()
    }

    fn parse_exp_atom(&mut self) -> AstNode {
        if let Some(token) = self.tok.peek() {
            match token {
                &Token::Sym(Symbol::LeftParen) => {
                    self.tok.consume_expect(&Token::Sym(Symbol::LeftParen));
                    let node = self.parse_exp();
                    self.tok.consume_expect(&Token::Sym(Symbol::LeftParen));
                    return node;
                }
                t => {
                    panic!("Unexpected token: {:?} at {}", t, self.tok.loc())
                }
            }
        } else {
            panic!("Unexpected EOF.")
        }
    }
}