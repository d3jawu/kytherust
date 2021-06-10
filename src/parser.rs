use crate::tokenizer::*;
use crate::tokenizer::Symbol::*;
use crate::tokenizer::Keyword::*;

use std::collections::HashMap;

use lazy_static::lazy_static;
use crate::parser::AstNode::Access;

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
    // necessary - a block is a node because it evaluates to a type
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
        result: Box<AstNode>,
    },
    Typeof {
        operand: Box<AstNode>,
    },
    Identifier(String),
    Access {
        target: Box<AstNode>,
        field: String,
    },
    // Import,
    // Export,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Unit,
    Int(i32),
    Double(f64),
    String(String),
    Bool(bool),
    Struct(HashMap<String, AstNode>),
    StructType(HashMap<String, AstNode>),
    Fn {
        param_names: Vec<String>,
        body: Box<AstNode>,
    },
    FnType {
        param_types: Vec<AstNode>,
        returns: Box<AstNode>,
    },
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
            Greater,
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

        // repeatedly attempt to grow the expression to the right until none are available
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
                // n literal, fn type literal, or parenthesized expression
                &Token::Sym(LeftParen) => {
                    self.tok.consume_expect(&Token::Sym(LeftParen));
                    if let Some(next_tok) = self.tok.peek() {
                        match next_tok {
                            // beginning of fn literal with no args
                            &Token::Sym(RightParen) => {
                                return self.parse_fn_literal(&mut Vec::new());
                            }
                            _ => {
                                // need to peek after the next exp, so hang on to it for now
                                let next_exp = self.parse_exp(true);

                                match self.tok.peek() {
                                    // identifier means fn literal:
                                    //    e.g. (int x) => {}
                                    // next_exp-^   ^-id we just peeked
                                    Some(Token::Id(_)) => {
                                        return self.parse_fn_literal(&mut vec![next_exp]);
                                    }
                                    // comma means fn type literal:
                                    //    e.g. (int, int,) => {}
                                    // next_exp-^  ^- comma we just peeked
                                    Some(Token::Sym(Comma)) => {
                                        panic!("fn type literal is not yet implemented.")
                                    }
                                    // next_exp is paren-wrapped expression
                                    //     e.g. (1);
                                    //  next_exp-^^- paren we just peeked
                                    Some(Token::Sym(RightParen)) => {
                                        self.tok.consume_expect(&Token::Sym(RightParen));
                                        return next_exp;
                                    }
                                    Some(_) => {
                                        panic!("Expected identifier, \",\", or \")\" {:?} at {}.", next_exp, self.tok.loc())
                                    }
                                    None => {
                                        panic!("Unexpected EOF.")
                                    }
                                }
                            }
                        };
                    } else {
                        panic!("Unexpected EOF.")
                    };
                }
                // list literal
                &Token::Sym(LeftBracket) => {
                    panic!("List literal is not yet implemented.")
                }
                // code block, struct literal, or struct type literal
                &Token::Sym(LeftBrace) => {
                    self.tok.consume_expect(&Token::Sym(LeftBrace));

                    // hang onto first expression after brace
                    let first_exp = self.parse_exp(true);
                    // look at token after first exp
                    match self.tok.peek() {
                        // colon means struct literal
                        &Some(Token::Sym(Colon)) => {
                            let mut result: HashMap<String, AstNode> = HashMap::new();

                            while let Some(token) = self.tok.peek() {
                                match token {
                                    // first run only, consume token following id in first_exp
                                    Token::Sym(Colon) => {
                                        self.tok.consume_expect(&Token::Sym(Colon));

                                        let exp = self.parse_exp(true);
                                        if let AstNode::Identifier(ref key) = first_exp {

                                            result.insert(key.clone(), exp);

                                            self.tok.consume_expect(&Token::Sym(Comma));
                                        } else {
                                            panic!("Expecting identifier for first entry in struct but got {:?} at {}.", first_exp, self.tok.loc())
                                        }
                                    }
                                    Token::Id(k) => {
                                        let key = k.to_string();

                                        // self.tok.consume_expect(&Token::Id(key));
                                        self.tok.consume();
                                        self.tok.consume_expect(&Token::Sym(Colon));

                                        let exp = self.parse_exp(true);
                                        result.insert(key, exp);

                                        self.tok.consume_expect(&Token::Sym(Comma));
                                    }
                                    &Token::Sym(RightBrace) => {
                                        break;
                                    }
                                    _ => {
                                        panic!("Expected struct field name or '}}' but got {:?} at {}", token, self.tok.loc())
                                    }
                                }
                            };

                            self.tok.consume_expect(&Token::Sym(RightBrace));
                            return AstNode::Literal(Literal::Struct(result));
                        }
                        // semicolon means code block
                        &Some(Token::Sym(Semicolon)) => {
                            self.tok.consume_expect(&Token::Sym(Semicolon));
                            return self.parse_started_block(first_exp);
                        }
                        // another expression means struct type literal
                        &Some(_) => {
                            // TODO implement
                            panic!("struct type literal not implemented")
                        }
                        None => {
                            panic!("Expected ':', ';', or expression but got EOF.");
                        }
                    };
                }
                // unary
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
                        operand: Box::from(self.parse_exp(true))
                    }
                }
                &Token::Kw(If) => {
                    panic!("'if' is not yet implemented.")
                }
                // declaration
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
                // control flow
                &Token::Kw(op) if op == Return || op == Continue || op == Break => {
                    self.tok.consume();
                    let next = if let Some(Token::Sym(Semicolon)) = self.tok.peek() {
                        // return without value implicitly returns unit
                        AstNode::Literal(Literal::Unit)
                    } else {
                        self.parse_exp(true)
                    };
                    return AstNode::Jump {
                        op,
                        result: Box::from(next),
                    };
                }
                // int literal
                &Token::Int(n) => {
                    self.tok.consume_expect(&Token::Int(n));
                    AstNode::Literal(Literal::Int(n))
                }
                // double literal
                &Token::Double(d) => {
                    self.tok.consume_expect(&Token::Double(d));
                    AstNode::Literal(Literal::Double(d))
                }
                // built-in constants
                Token::Id(_) => {
                    if let Some(Token::Id(id)) = self.tok.consume() {
                        match id.as_str() {
                            "true" => {
                                AstNode::Literal(Literal::Bool(true))
                            }
                            "false" => {
                                AstNode::Literal(Literal::Bool(false))
                            }
                            "unit" => {
                                AstNode::Literal(Literal::Unit)
                            }
                            // TODO: built-in type literals (Int, Double, etc)
                            id => {
                                AstNode::Identifier(String::from(id))
                            }
                        }
                    } else {
                        panic!("Expecting identifier but got {:?} at {}", self.tok.peek(), self.tok.loc())
                    }
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
                return lhs;
            }

            let next_precedence = OP_PRECEDENCE[&op];
            if next_precedence > precedence {
                self.tok.consume();
                let rhs_exp = self.parse_exp(true);
                let rhs = self.make_binary(rhs_exp, next_precedence);

                let binary = AstNode::Binary {
                    lhs: Box::from(lhs),
                    rhs: Box::from(rhs),
                    op,
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
        self.tok.consume_expect(&Token::Sym(Dot));
        if let Some(Token::Id(f)) = self.tok.peek() {
            let field = f.clone();
            self.tok.consume_expect(&Token::Id(field.to_string()));
            Access {
                target: Box::from(target),
                field,
            }
        } else {
            panic!("Expected field identifier but got {:?} at {}", self.tok.peek(), self.tok.loc());
        }
    }

    fn make_bracket_access(&mut self, _target: AstNode) -> AstNode {
        panic!("Bracket access is not is not yet implemented.")
    }

    // the parser should have already consumed the left-paren
    // param_types may be pre-populated with the type exp of the first param
    fn parse_fn_literal(&mut self, param_types: &mut Vec<AstNode>) -> AstNode {
        let mut param_names: Vec<String> = Vec::new();

        // if paramTypes is pre-populated, the next token is the param name and we need to grab that.
        if param_types.len() == 1 {
            if let Some(Token::Id(n)) = self.tok.peek() {
                let name = n.clone();
                param_names.push(name.clone());
                self.tok.consume_expect(&Token::Id(name));
            } else {
                panic!("Expected parameter name but got {:?} at {}", self.tok.peek(), self.tok.loc())
            }

            // optionally consume comma
            if let Some(Token::Sym(sym)) = self.tok.peek() {
                match sym {
                    Comma => {
                        self.tok.consume_expect(&Token::Sym(Comma));
                    }
                    RightParen => {}
                    _ => {
                        panic!("Expected \",\" or \")\" after parameter but got {:?} at {}", self.tok.peek(), self.tok.loc())
                    }
                };
            }
        }

        loop {
            if let Some(Token::Sym(RightParen)) = self.tok.peek() {
                break;
            }

            let param_type = self.parse_exp(true);
            if let Some(Token::Id(p_n)) = self.tok.peek() {
                let param_name = p_n.clone();
                param_names.push(param_name.clone());
                self.tok.consume_expect(&Token::Id(param_name));
                param_types.push(param_type);
                // optionally consume comma
                if let Some(Token::Sym(sym)) = self.tok.peek() {
                    match sym {
                        Comma => {
                            self.tok.consume_expect(&Token::Sym(Comma));
                        }
                        RightParen => {}
                        _ => {
                            panic!("Expected \",\" or \")\" after parameter but got {:?} at {}", self.tok.peek(), self.tok.loc())
                        }
                    };
                }
            } else {
                panic!("Expecting parameter name but got {:?} at {}", self.tok.peek(), self.tok.loc())
            }
        }

        self.tok.consume_expect(&Token::Sym(RightParen));

        self.tok.consume_expect(&Token::Sym(Equal));
        self.tok.consume_expect(&Token::Sym(Greater));

        let body = self.parse_block();

        AstNode::Literal(Literal::Fn {
            param_names,
            body: Box::from(body),
        })
    }

    fn parse_block(&mut self) -> AstNode {
        self.tok.consume_expect(&Token::Sym(LeftBrace));

        let first_stmt = self.parse_exp(true);
        self.tok.consume_expect(&Token::Sym(Semicolon));

        return self.parse_started_block(first_stmt);
    }

    // parses block with first brace, statement, and semicolon already consumed.
    // only uniquely used in parse_exp_atom, but made a function to factor out common code.
    // see &Token::Sym(LeftBrace) match entry in parse_exp_atom
    fn parse_started_block(&mut self, first_stmt: AstNode) -> AstNode {
        let mut body: Vec<AstNode> = Vec::new();

        body.push(first_stmt);

        loop {
            if let Some(Token::Sym(RightBrace)) = self.tok.peek() {
                break;
            }

            body.push(self.parse_exp(true));
            self.tok.consume_expect(&Token::Sym(Semicolon));
        }

        self.tok.consume_expect(&Token::Sym(RightBrace));

        AstNode::Block {
            body,
        }
    }
}
