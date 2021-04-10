use crate::tokenizer::*;
use crate::tokenizer::Symbol::*;
use crate::tokenizer::Keyword::*;
use crate::parser::AstNode::Declaration;
use std::borrow::Borrow;

pub struct Parser {
    tok: Tokenizer,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    Assign {
        id: String,
        rhs: Box<AstNode>,
    },
    Binary {
        lhs: Box<AstNode>,
        op: Symbol,
        rhs: Box<AstNode>,
    },
    Unary {
        op: Symbol,
        operand: Box<AstNode>,
    },
    Call {
        target: Box<AstNode>,
        arguments: Vec<AstNode>,
    },
    Block {
        body: Vec<AstNode>,
    },
    Literal(Literal),
    Declaration {
        op: Keyword,
        id: String,
        value: Box<AstNode>,
    },
    If {
        condition: Box<AstNode>,
        body: Box<AstNode>,
        else_body: Option<Box<AstNode>>,
    },
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    When,
    Jump {
        op: Keyword,
    },
    // break, return, continue
    Typeof {
        operand: Box<AstNode>,
    },
    Identifier(String),
    // Import,
    // Export,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(i32),
    Double(f64),
    String(String),
    Bool(bool),
    Struct,
    Fn,
}

fn is_binary(token: &Token) -> bool {
    if let Token::Sym(sym) = token {
        [
            Bar,
            And,
            AndAnd,
            BarBar,
            EqualEqual,
            BangEqual,
            Less,
            LessEqual,
            GreaterEqual,
            Plus,
            Minus,
            Star,
            Slash,
            Percent,
        ]
            .contains(sym)
    } else {
        false
    }
}

impl Parser {
    pub fn new(tok: Tokenizer) -> Parser {
        Parser { tok }
    }

    // parse until EOF
    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut program: Vec<AstNode> = Vec::new();
        while self.tok.peek().is_some() {
            program.push(self.parse_exp(true));

            self.tok.consume_expect(&Token::Sym(Semicolon));
        }

        program
    }

    // assemble composite expressions, e.g. binary exps, function calls, etc
    fn parse_exp(&mut self, compose: bool) -> AstNode {
        let exp = self.parse_exp_atom();

        if !compose {
            return exp;
        }

        let mut composed: AstNode = exp;

        loop {
            let next = self.tok.peek();
            let mut finished = false;

            match next {
                Some(t) => {
                    match t {
                        t if t == &Token::Sym(LeftParen) => {
                            composed = self.make_call(composed);
                        }
                        t if is_binary(t) => {
                            composed = self.make_binary(composed);
                        }
                        t if t == &Token::Sym(Dot) => {
                            composed = self.make_dot_access(composed);
                        }
                        t if t == &Token::Sym(LeftBracket) => {
                            composed = self.make_bracket_access(composed);
                        }
                        _ => {
                            finished = true;
                        }
                    }
                }
                None => {
                    panic!("Unexpected EOF.")
                }
            }

            if finished {
                break;
            }
        }

        composed
    }

    fn parse_exp_atom(&mut self) -> AstNode {
        if let Some(token) = self.tok.peek() {
            match token {
                &Token::Sym(LeftParen) => {
                    self.tok.consume_expect(&Token::Sym(LeftParen));
                    let node = self.parse_exp(true);
                    self.tok.consume_expect(&Token::Sym(RightParen));
                    node
                }
                &Token::Sym(LeftBracket) => {
                    panic!("List literal not yet implemented.")
                }
                &Token::Sym(Bang) => {
                    self.tok.consume_expect(&Token::Sym(Bang));
                    AstNode::Unary {
                        op: Bang,
                        operand: Box::from(self.parse_exp_atom()),
                    }
                }
                &Token::Kw(Typeof) => {
                    self.tok.consume_expect(&Token::Kw(Typeof));
                    AstNode::Typeof {
                        operand: Box::from(self.parse_exp_atom())
                    }
                }
                &Token::Kw(If) => {
                    panic!("'if' is not yet implemented.")
                }
                &Token::Kw(kw) if kw == Const || kw == Let => {
                    self.tok.consume();
                    if let Some(Token::Id(id)) = self.tok.consume() {
                        self.tok.consume_expect(&Token::Sym(Equal));
                        return Declaration {
                            op: kw,
                            id,
                            value: Box::from(self.parse_exp_atom()),
                        };
                    } else {
                        panic!("Expecting identifier but got {:?} at {}", self.tok.peek(), self.tok.loc())
                    }
                }
                &Token::Int(n) => {
                    self.tok.consume_expect(&Token::Int(n));
                    AstNode::Literal(Literal::Int(n))
                }
                &Token::Double(d) => {
                    self.tok.consume_expect(&Token::Double(d));
                    AstNode::Literal(Literal::Double(d))
                }
                t => {
                    panic!("Unexpected token: {:?} at {}", t, self.tok.loc())
                }
            }
        } else {
            panic!("Unexpected EOF.")
        }
    }

    fn make_binary(&mut self, lhs: AstNode) -> AstNode {
        panic!()
    }

    fn make_call(&mut self, target: AstNode) -> AstNode {
        panic!()
    }

    fn make_dot_access(&mut self, target: AstNode) -> AstNode {
        panic!()
    }

    fn make_bracket_access(&mut self, target: AstNode) -> AstNode {
        panic!()
    }
}
