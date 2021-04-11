use crate::tokenizer::*;
use crate::tokenizer::Symbol::*;
use crate::tokenizer::Keyword::*;

use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref OP_PRECEDENCE: HashMap<Symbol, u8> = {
        let mut map = HashMap::new();

        // TODO macro to factor out the repeated code
        map.insert(Equal, 1);
        map.insert(PlusEqual, 1);
        map.insert(MinusEqual, 1);
        map.insert(StarEqual, 1);
        map.insert(SlashEqual, 1);
        map.insert(PercentEqual, 1);

        map.insert(BarBar, 3);
        map.insert(AndAnd, 4);

        map.insert(EqualEqual,8);
        map.insert(BangEqual,8);

        map.insert(Less,9);
        map.insert(LessEqual,9);
        map.insert(Greater,9);
        map.insert(GreaterEqual,9);

        map.insert(Plus,11);
        map.insert(Minus,11);

        map.insert(Star,12);
        map.insert(Slash,12);
        map.insert(Percent,12);

        map.insert(Bang,14);

        map.insert(Dot,16);
        map.insert(LeftParen,16);
        map.insert(RightParen,16);
        map.insert(LeftBracket,16);
        map.insert(RightBracket,16);
        map.insert(LeftBrace,16);
        map.insert(RightBrace,16);

        map
    };
}

pub struct Parser {
    tok: Tokenizer,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    // assignment is considered a binary node
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
    // break, return, continue
    Jump {
        op: Keyword,
    },
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
                            composed = self.make_binary(composed, 0);
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
                        return AstNode::Declaration {
                            op: kw,
                            id,
                            value: Box::from(self.parse_exp(true)),
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

    fn make_binary(&mut self, lhs: AstNode, precedence: u8) -> AstNode {
        if let &Some(Token::Sym(op)) = self.tok.peek() {
            if !OP_PRECEDENCE.contains_key(&op) {
                // not a binary op, just return lhs
                return lhs
            }

            let next_precedence = OP_PRECEDENCE[&op];
            if next_precedence > precedence {
                self.tok.consume();
                let rhs_exp = self.parse_exp(false);
                let rhs = self.make_binary(rhs_exp, next_precedence);

                let binary = AstNode::Binary {
                    lhs: Box::from(lhs),
                    rhs: Box::from(rhs),
                    op
                };

                return self.make_binary(binary, precedence);
            }
        }

        lhs
    }

    fn make_call(&mut self, target: AstNode) -> AstNode {
        let mut args: Vec<AstNode> = Vec::new();
        self.tok.consume_expect(&Token::Sym(LeftParen));

        while let Some(token) = self.tok.peek() {
            if token == &Token::Sym(RightParen) {
                break;
            }

            args.push(self.parse_exp(true));

            // trailing comma optional
            if let Some(Token::Sym(RightParen)) = self.tok.peek() {
                break;
            } else {
                self.tok.consume_expect(&Token::Sym(Comma));
            }
        }

        self.tok.consume_expect(&Token::Sym(RightParen));

        AstNode::Call {
            arguments: args,
            target: Box::from(target),
        }
    }

    fn make_dot_access(&mut self, target: AstNode) -> AstNode {
        panic!()
    }

    fn make_bracket_access(&mut self, target: AstNode) -> AstNode {
        panic!()
    }
}
