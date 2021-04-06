use crate::input_stream::InputStream;

#[derive(Debug, PartialEq)]
pub enum Token {
    Str(String), // string literal
    Sym(Symbol), // symbol
    Int(i32),    // integer literal
    Double(f64), // FP literal
    Kw(Keyword), // language-defined keyword
    Id(String),  // user-defined identifier
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Const,
    Let,
    If,
    Else,
    While,
    When,
    Break,
    Return,
    Continue,
    Typeof,
    Import,
    Export,
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,

    Bar,
    And,
    BarBar,
    AndAnd,

    EqualEqual,
    // EqualEqualEqual,
    BangEqual,
    // BangEqualEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Bang,

    Dot,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Comma,
    Semicolon,
    Colon,
}
pub struct Tokenizer {
    current: Option<Token>,
    stream: InputStream,
}

macro_rules! as_char {
    ($s:expr) => {
        $s.chars().next().unwrap_or('\0')
    };
}

macro_rules! some_sym_tok {
    ($sym:ident) => {
        Some(Token::Sym(Symbol::$sym))
    };
}

macro_rules! some_kw_tok {
    ($kw:ident) => {
        Some(Token::Kw(Keyword::$kw))
    };
}

impl Tokenizer {
    pub fn new(stream: InputStream) -> Tokenizer {
        let mut tok = Tokenizer {
            current: None,
            stream,
        };

        // step into first token
        tok.advance();

        tok
    }

    pub fn loc(&self) -> String {
        self.stream.loc()
    }

    // read current token without consuming it
    pub fn peek(&self) -> &Option<Token> {
        &self.current
    }

    // read current token and move to next
    pub fn consume(&mut self) -> Option<Token> {
        let current = self.current.take();
        self.advance();

        current
    }

    // like peek, but panic if expected is not present
    pub fn expect(&self, expected: &Token) -> &Option<Token> {
        let t = self.peek();

        if t.as_ref() != Some(expected) {
            panic!(
                "Expected {:?} but got {:?} at {}.",
                expected,
                t,
                self.stream.loc()
            );
        }

        t
    }

    // like consume, but panic if expected is not present
    pub fn consume_expect(&mut self, expected: &Token) -> Option<Token> {
        self.expect(expected);
        self.consume()
    }

    // parse one token from input stream
    fn advance(&mut self) {
        // clear non-tokens (whitespace and comments)
        loop {
            match self.stream.peek() {
                // possibly a comment
                Some(c) if c == "/" => {
                    match self.stream.peek_next().unwrap_or("".to_string()).as_str() {
                        // multi-line comment
                        "*" => loop {
                            if let (Some(n), Some(m)) =
                                (self.stream.peek(), self.stream.peek_next())
                            {
                                if n == "*" && m == "/" {
                                    self.stream.consume_expect("*");
                                    self.stream.consume_expect("/");
                                    break;
                                }
                            }

                            if self.stream.eof() {
                                panic!("Unexpected EOF.")
                            }

                            self.stream.consume();
                        },
                        // single-line comment
                        "/" => {
                            self.stream.read_while(|s| s != "\n");
                            self.stream.consume_expect("\n");
                        }
                        // anything else, treat it as a / token and continue to token parsing
                        _ => break,
                    };
                }
                // whitespace
                Some(c) if as_char!(c).is_whitespace() => {
                    self.stream.read_while(|s| as_char!(s).is_whitespace());
                }
                _ => break, // no non-tokens ahead, move on to actual parsing
            }
        }

        // check for EOF
        if self.stream.eof() {
            self.current = None;
            return;
        }

        macro_rules! sym_or_sym_and {
            ($and:expr, $sym_name:ident, $sym_and_name:ident) => {
                if let Some(c) = self.stream.peek() {
                    match c.as_str() {
                        $and => {
                            self.stream.consume_expect($and);
                            some_sym_tok!($sym_and_name)
                        }
                        _ => some_sym_tok!($sym_name), // if anything else, treat it as just sym
                    }
                } else {
                    panic!("Unexpected EOF.")
                }
            };
        }

        // at this point, there is definitely a token ahead
        self.current = match self.stream.consume().as_str() {
            // string literal
            "\"" => {
                // TODO escaped
                let val = self.stream.read_while(|s| s != "\"");

                // eat "
                self.stream.consume_expect("\"");

                Some(Token::Str(val))
            }
            // symbols
            "+" => sym_or_sym_and!("=", Plus, PlusEqual),
            "-" => sym_or_sym_and!("=", Minus, MinusEqual),
            "*" => sym_or_sym_and!("=", Star, StarEqual),
            "/" => sym_or_sym_and!("=", Slash, SlashEqual),
            "%" => sym_or_sym_and!("=", Percent, PercentEqual),

            "|" => sym_or_sym_and!("|", Bar, BarBar),
            "&" => sym_or_sym_and!("&", And, AndAnd),

            "=" => sym_or_sym_and!("=", Equal, EqualEqual),
            "!" => sym_or_sym_and!("=", Bang, BangEqual),

            "<" => sym_or_sym_and!("=", Less, LessEqual),
            ">" => sym_or_sym_and!("=", Greater, GreaterEqual),

            "." => some_sym_tok!(Dot),
            "(" => some_sym_tok!(LeftParen),
            ")" => some_sym_tok!(RightParen),
            "{" => some_sym_tok!(LeftBrace),
            "}" => some_sym_tok!(RightBrace),
            "[" => some_sym_tok!(LeftBracket),
            "]" => some_sym_tok!(RightBracket),

            "," => some_sym_tok!(Comma),
            ";" => some_sym_tok!(Semicolon),
            ":" => some_sym_tok!(Colon),
            t if as_char!(t).is_digit(10) => {
                let mut output: Vec<String> = vec![t.to_string()];
                let mut has_dec = false;

                while let Some(s) = self.stream.peek() {
                    if as_char!(s).is_digit(10) || s == "." {
                        if s == "." {
                            if has_dec {
                                panic!("Unexpected '.' at {}", self.stream.loc())
                            }

                            has_dec = true;
                        }

                        self.stream.consume_expect(&s);
                        output.push(s);
                    } else {
                        break;
                    }
                }

                let result = output.join("");

                if has_dec {
                    Some(Token::Double(result.parse::<f64>().unwrap_or_else(|_| {
                        panic!("Could not parse number: {}", result)
                    })))
                } else {
                    Some(Token::Int(result.parse::<i32>().unwrap_or_else(|_| {
                        panic!("Could not parse integer: {}", result)
                    })))
                }
            }
            t => {
                // read as whole word
                let word = {
                    let mut word: Vec<String> = vec![t.to_string()];

                    while let Some(s) = self.stream.peek() {
                        if !as_char!(s).is_alphanumeric() && s != "_" {
                            break;
                        }

                        self.stream.consume_expect(&s);
                        word.push(s);
                    }

                    word.join("")
                };

                match word.as_str() {
                    "const" => some_kw_tok!(Const),
                    "let" => some_kw_tok!(Let),
                    "if" => some_kw_tok!(If),
                    "else" => some_kw_tok!(Else),
                    "while" => some_kw_tok!(While),
                    "when" => some_kw_tok!(When),
                    "break" => some_kw_tok!(Break),
                    "return" => some_kw_tok!(Return),
                    "continue" => some_kw_tok!(Continue),
                    "typeof" => some_kw_tok!(Typeof),
                    "import" => some_kw_tok!(Import),
                    "export" => some_kw_tok!(Export),
                    // user-identified keyword
                    id => Some(Token::Id(id.to_string())),
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_stream;
    use std::io::Error;

    #[test]
    fn tokens() -> Result<(), String> {
        let input = "\"string literal\"
= += -= *= /= %=
| & || &&
== != < > <= >=
+ - * / %
!
. ( ) [ ] { }
, ; :

42
3.14159

const
let
if
else
while
when
break
return
continue
typeof
import
export

myVar";

        let mut tokenizer = Tokenizer::new(input_stream::InputStream::new_from_string(input));

        let expected = vec![
            Some(Token::Str("string literal".to_string())),
            some_sym_tok!(Equal),
            some_sym_tok!(PlusEqual),
            some_sym_tok!(MinusEqual),
            some_sym_tok!(StarEqual),
            some_sym_tok!(SlashEqual),
            some_sym_tok!(PercentEqual),
            some_sym_tok!(Bar),
            some_sym_tok!(And),
            some_sym_tok!(BarBar),
            some_sym_tok!(AndAnd),
            some_sym_tok!(EqualEqual),
            some_sym_tok!(BangEqual),
            some_sym_tok!(Less),
            some_sym_tok!(Greater),
            some_sym_tok!(LessEqual),
            some_sym_tok!(GreaterEqual),
            some_sym_tok!(Plus),
            some_sym_tok!(Minus),
            some_sym_tok!(Star),
            some_sym_tok!(Slash),
            some_sym_tok!(Percent),
            some_sym_tok!(Bang),
            some_sym_tok!(Dot),
            some_sym_tok!(LeftParen),
            some_sym_tok!(RightParen),
            some_sym_tok!(LeftBracket),
            some_sym_tok!(RightBracket),
            some_sym_tok!(LeftBrace),
            some_sym_tok!(RightBrace),
            some_sym_tok!(Comma),
            some_sym_tok!(Semicolon),
            some_sym_tok!(Colon),
            Some(Token::Int(42)),
            Some(Token::Double(3.14159)),
            some_kw_tok!(Const),
            some_kw_tok!(Let),
            some_kw_tok!(If),
            some_kw_tok!(Else),
            some_kw_tok!(While),
            some_kw_tok!(When),
            some_kw_tok!(Break),
            some_kw_tok!(Return),
            some_kw_tok!(Continue),
            some_kw_tok!(Typeof),
            some_kw_tok!(Import),
            some_kw_tok!(Export),
            Some(Token::Id("myVar".to_string()))
        ];
        let mut expected_iter = expected.iter();

        // assert_eq!(tokenizer.consume(), Some(Token::Str("string literal".to_string())));

        while let (got, Some(expected)) = (tokenizer.consume(), expected_iter.next()) {
            println!("{:?} vs {:?}", got, expected);
            assert_eq!(&got, expected);
        }

        Ok(())
    }
}
