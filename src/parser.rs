use crate::tokenizer::*;

pub struct Parser {
    tok: Tokenizer,
}

#[derive(Debug, PartialEq)]
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
    }, // break, return, continue
    Typeof {
        operand: Box<AstNode>,
    },
    Identifier(String),
    // Import,
    // Export,
}

fn is_binary(token: &Token) -> bool {
    if let Token::Sym(sym) = token {
        [
            Symbol::Bar,
            Symbol::And,
            Symbol::AndAnd,
            Symbol::BarBar,
            Symbol::EqualEqual,
            Symbol::BangEqual,
            Symbol::Less,
            Symbol::LessEqual,
            Symbol::GreaterEqual,
            Symbol::Plus,
            Symbol::Minus,
            Symbol::Star,
            Symbol::Slash,
            Symbol::Percent,
        ]
        .contains(sym)
    } else {
        false
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    Double(f64),
    String(String),
    Bool(bool),
    Struct,
    Fn,
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

            self.tok.consume_expect(&Token::Sym(Symbol::Semicolon));
        }

        program
    }

    // assemble composite expressions, e.g. binary exps, function calls, etc
    fn parse_exp(&mut self, compose: bool) -> AstNode {
        let exp = self.parse_exp_atom();

        if !compose {
            return exp
        }


        let mut composed: AstNode = exp;

        loop {
            let next = self.tok.peek();

            let mut finished = true;

            if next.is_none() {
                panic!()
            }

            match next {
                Some(t) => {
                    match t {
                        t if is_binary(t) => {
                            composed = self.make_binary(composed);
                            finished = false;
                        },
                        t if t == &Token::Sym(Symbol::Dot) => {
                            composed = self.make_dot_access(composed);
                            finished = false;
                        },
                        t if t == &Token::Sym(Symbol::LeftParen) => {
                            composed = self.make_call(composed);
                            finished = false;
                        },
                        t if t == &Token::Sym(Symbol::LeftBracket) => {
                            panic!("Bracket access not yet implemented.")
                        }
                        t => {
                            panic!("Unexpected token {:?} at {}.", t, self.tok.loc())
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


        panic!()
    }

    fn parse_exp_atom(&mut self) -> AstNode {
        if let Some(token) = self.tok.peek() {
            match token {
                &Token::Sym(Symbol::LeftParen) => {
                    self.tok.consume_expect(&Token::Sym(Symbol::LeftParen));
                    let node = self.parse_exp(true);
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

    fn make_binary(&mut self, lhs: AstNode) -> AstNode {
        panic!()
    }

    fn make_call(&mut self, target: AstNode) -> AstNode {
        panic!()
    }

    fn make_dot_access(&mut self, target: AstNode) -> AstNode {
        panic!()
    }
}
