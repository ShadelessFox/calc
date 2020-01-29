type Value = f64;

// This is a bad way to define built-in functions, but for a simple calculator it's ok
static FUNCTIONS: [(&'static str, u8, fn(Vec<Value>) -> Value); 15] = [
    ("pi", 0, |_| std::f64::consts::PI),
    ("e", 0, |_| std::f64::consts::E),
    ("golden", 0, |_| 1.618033988749895f64),
    ("abs", 1, |args| args[0].abs()),
    ("sin", 1, |args| args[0].sin()),
    ("cos", 1, |args| args[0].cos()),
    ("tan", 1, |args| args[0].tan()),
    ("round", 1, |args| args[0].round()),
    ("floor", 1, |args| args[0].floor()),
    ("ceil", 1, |args| args[0].ceil()),
    ("fib", 1, |args| (1.618033988749895f64.powf(args[0]) / 2.23606797749979f64).round()),
    ("sqrt", 1, |args| args[0].sqrt()),
    ("log", 2, |args| args[1].log(args[0])),
    ("min", 2, |args| args[0].min(args[1])),
    ("max", 2, |args| args[0].max(args[1])),
];

#[derive(PartialEq, Clone)]
enum Token {
    Value(Value),
    Name(String),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    BrOpen,
    BrClose,
    End,
}

impl Token {
    fn as_value(&self) -> Value {
        match *self {
            Token::Value(ref x) => *x,
            _ => panic!("Not a number"),
        }
    }

    fn as_name(&self) -> String {
        match *self {
            Token::Name(ref x) => x.clone(),
            _ => panic!("Not a name"),
        }
    }

    fn is_value(&self) -> bool {
        match *self {
            Token::Value(_) => true,
            _ => false,
        }
    }

    fn is_name(&self) -> bool {
        match *self {
            Token::Name(_) => true,
            _ => false,
        }
    }

    fn is_unary_operator(&self) -> bool {
        match *self {
            Token::Add | Token::Sub => true,
            _ => false,
        }
    }

    fn is_binary_operator(&self) -> bool {
        match *self {
            Token::Add | Token::Sub | Token::Mul | Token::Div | Token::Pow | Token::Mod => true,
            _ => false,
        }
    }

    fn apply_binary(&self, left: Value, right: Value) -> Value {
        match *self {
            Token::Add => left + right,
            Token::Sub => left - right,
            Token::Mul => left * right,
            Token::Div => left / right,
            Token::Pow => left.powf(right),
            Token::Mod => left % right,
            _ => panic!("Not a binary operator")
        }
    }

    fn apply_unary(&self, right: Value) -> Value {
        match *self {
            Token::Add => right,
            Token::Sub => -right,
            _ => panic!("Not an unary operator")
        }
    }

    fn get_precedence(&self) -> u32 {
        match *self {
            Token::Add | Token::Sub => 1,
            Token::Mul | Token::Div | Token::Pow | Token::Mod => 2,
            _ => panic!("Not an operator")
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Token::Value(x) => write!(f, "{}", x),
            Token::Name(x) => write!(f, "{}", x),
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Mul => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Pow => write!(f, "^"),
            Token::Mod => write!(f, "%"),
            Token::BrOpen => write!(f, "("),
            Token::BrClose => write!(f, ")"),
            Token::End => write!(f, "<EOF>"),
        }
    }
}

enum Node {
    Const(Value),
    UnExpr(Box<Node>, Token),
    BiExpr(Box<Node>, Box<Node>, Token),
}

impl Node {
    fn execute(&self) -> Value {
        match self {
            Node::Const(value) => *value,
            Node::UnExpr(ref right, op) => op.apply_unary(right.execute()),
            Node::BiExpr(ref left, ref right, op) => op.apply_binary(left.execute(), right.execute()),
        }
    }
}

struct Lexer {
    buffer: String,
    offset: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer { buffer: input, offset: 0 }
    }

    fn tokenize(&mut self) -> Result<Token, String> {
        loop {
            let next = self.char_read().unwrap_or_default();

            if next.is_whitespace() {
                continue;
            } else if next.is_ascii_digit() {
                let mut buffer = next.to_string();
                let mut seen_point = false;

                loop {
                    let digit = self.char_peek(0).unwrap_or_default();

                    if digit == '.' {
                        if seen_point {
                            break;
                        } else {
                            seen_point = true;
                        }
                    } else if !digit.is_ascii_digit() {
                        break;
                    }

                    buffer.push(self.char_read()?);
                }

                return Ok(Token::Value(buffer.parse::<f64>().unwrap()));
            } else if next.is_alphabetic() {
                let mut buffer = next.to_string();

                loop {
                    let character = self.char_peek(0).unwrap_or_default();

                    if character.is_alphabetic() {
                        buffer.push(self.char_read()?);
                    } else {
                        break;
                    }
                }

                return Ok(Token::Name(buffer));
            } else if next == '+' {
                return Ok(Token::Add);
            } else if next == '-' {
                return Ok(Token::Sub);
            } else if next == '*' {
                return Ok(Token::Mul);
            } else if next == '/' {
                return Ok(Token::Div);
            } else if next == '^' {
                return Ok(Token::Pow);
            } else if next == '%' {
                return Ok(Token::Mod);
            } else if next == '(' {
                return Ok(Token::BrOpen);
            } else if next == ')' {
                return Ok(Token::BrClose);
            } else if next == char::default() {
                return Ok(Token::End);
            } else {
                return Err(format!("Unexpected character '{}'", next));
            }
        }
    }

    fn char_peek(&self, offset: usize) -> Result<char, String> {
        if self.offset + offset < self.buffer.len() {
            Ok(self.buffer.chars().nth(self.offset + offset).unwrap())
        } else {
            Err("Unexpected end of stream".to_string())
        }
    }

    fn char_read(&mut self) -> Result<char, String> {
        let peek = self.char_peek(0)?;
        self.offset += 1;
        Ok(peek)
    }
}

struct Parser {
    tokens: Vec<Token>,
    offset: usize,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Result<Self, String> {
        let mut tokens = Vec::new();

        loop {
            let token = lexer.tokenize()?;

            if token == Token::End {
                break;
            }

            tokens.push(token);
        }

        Ok(Parser { tokens, offset: 0 })
    }

    fn parse(&mut self) -> Result<Node, String> {
        let left = self.parse_unary()?;
        self.parse_binary(left, 0)
    }

    // https://en.wikipedia.org/wiki/Operator-precedence_parser
    fn parse_binary(&mut self, mut left: Node, min_precedence: u32) -> Result<Node, String> {
        let mut lookahead = self.token_peek(0);

        while lookahead.is_binary_operator() && lookahead.get_precedence() >= min_precedence {
            let operator = self.token_read(); // operator == lookahead
            let mut right = self.parse_unary()?;
            lookahead = self.token_peek(0);

            while lookahead.is_binary_operator() && lookahead.get_precedence() > operator.get_precedence() {
                right = self.parse_binary(right, lookahead.get_precedence())?;
                lookahead = self.token_peek(0);
            }

            left = Node::BiExpr(Box::from(left), Box::from(right), operator)
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Node, String> {
        let token = self.token_peek(0);

        if token.is_unary_operator() {
            self.token_read();
            Ok(Node::UnExpr(Box::from(self.parse_primary()?), token))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Node, String> {
        let token = self.token_read();

        if token == Token::BrOpen {
            let expr = self.parse()?;
            self.token_expect(Token::BrClose)?;
            Ok(expr)
        } else if token.is_value() {
            Ok(Node::Const(token.as_value()))
        } else if token.is_name() {
            let name = token.as_name();

            match FUNCTIONS.iter().find(|x| x.0 == name) {
                Some((_, count, func)) => {
                    let mut args = Vec::new();

                    for _ in 0..*count {
                        args.push(self.parse_primary()?.execute());
                    }

                    Ok(Node::Const((*func)(args)))
                }
                None => Err(format!("No such function defined: '{}'", name)),
            }
        } else {
            Err(format!("Unexpected token '{}'", token))
        }
    }

    fn token_expect(&mut self, expect: Token) -> Result<Token, String> {
        let token = self.token_read();

        if token == expect {
            Ok(token)
        } else {
            Err(format!("Expected '{}' but found '{}'", expect, token))
        }
    }

    fn token_peek(&self, offset: usize) -> Token {
        if self.offset + offset < self.tokens.len() {
            self.tokens[self.offset + offset].clone()
        } else {
            Token::End
        }
    }

    fn token_read(&mut self) -> Token {
        let peek = self.token_peek(0);
        self.offset += 1;
        peek
    }
}

fn read(input: String) -> Result<Node, String> {
    Parser::new(Lexer::new(input))?.parse()
}

fn main() {
    use std::io::{stdout, stdin, BufReader, BufRead, Write};

    loop {
        stdout().lock().write(b"> ").unwrap();
        stdout().flush().unwrap();

        let mut buffer = String::new();
        BufReader::new(stdin()).read_line(&mut buffer).unwrap();

        match read(buffer) {
            Ok(x) => println!("{}", x.execute()),
            Err(x) => eprintln!("{}", x),
        }
    }
}