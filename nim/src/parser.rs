use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> Expr {
    let mut tokens = tokens.iter().peekable();
    parse_assignment(&mut tokens)
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    String(String),
    Variable(String),
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: char,
        right: Box<Expr>,
    },
    ArrayAccess {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    Array(Vec<Expr>),
}

fn parse_function_call(
    name: String,
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Expr {
    let mut args = Vec::new();

    if let Some(Token::LeftParen) = tokens.next() {
        while let Some(token) = tokens.peek() {
            match token {
                Token::RightParen => {
                    tokens.next();
                    break;
                }
                Token::Comma => {
                    tokens.next();
                }
                _ => {
                    let arg = parse_expression(tokens);
                    args.push(arg);
                }
            }
        }
    } else {
        panic!("Expected '(' after function name");
    }

    Expr::FunctionCall { name, args }
}

fn parse_assignment(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Expr {
    if let Some(Token::Identifier(name)) = tokens.peek() {
        if let Some(Token::Operator('=')) = tokens.nth(1) {
            let name = name.clone();
            let value = parse_expression(tokens);
            return Expr::Assignment {
                name: name.clone(),
                value: Box::new(value),
            };
        }
    }
    parse_expression(tokens)
}

pub fn parse_expression(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Expr {
    if let Some(Token::Functions(name)) = tokens.peek() {
        let name = name.clone();
        tokens.next();
        return parse_function_call(name.to_string(), tokens);
    }

    let mut left = parse_term(tokens);

    while let Some(Token::Operator(op)) = tokens.peek() {
        if *op == '+' || *op == '-' {
            tokens.next();
            let right = parse_term(tokens);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: *op,
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    left
}

fn parse_term(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Expr {
    let mut left = parse_factor(tokens);

    while let Some(Token::Operator(op)) = tokens.peek() {
        if *op == '*' || *op == '/' || *op == '^' {
            tokens.next();
            let right = parse_factor(tokens);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: *op,
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    left
}

pub fn parse_factor(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Expr {
    match tokens.next() {
        Some(Token::Integer(value)) => Expr::Number(*value as f64),
        Some(Token::Float(value)) => Expr::Number(*value),
        Some(Token::Str(value)) => Expr::String(value.to_string()),
        Some(Token::Identifier(name)) => {
            if let Some(Token::LeftBracket) = tokens.peek() {
                tokens.next();
                let index = parse_expression(tokens);
                if let Some(Token::RightBracket) = tokens.peek() {
                    tokens.next();
                    return Expr::ArrayAccess {
                        array: Box::new(Expr::Variable(name.clone())),
                        index: Box::new(index),
                    };
                }
            }
            Expr::Variable(name.clone())
        }
        Some(Token::LeftBracket) => {
            let mut elements = Vec::new();
            while let Some(token) = tokens.peek() {
                if matches!(token, Token::RightBracket) {
                    tokens.next();
                    break;
                }
                elements.push(parse_expression(tokens));
                if let Some(Token::Comma) = tokens.peek() {
                    tokens.next();
                }
            }
            Expr::Array(elements)
        }
        _ => panic!("Expected a number or variable"),
    }
}