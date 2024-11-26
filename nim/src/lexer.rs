#[derive(Debug)]
pub enum Token {
    Integer(i64),          // Целое число
    Float(f64),            // Вещевственое число
    Str(String),           // Строка
    Identifier(String),    // Переменные (например, x, y, _temp_)
    Functions(String),     // Функции (например, print)
    Operator(char),        // Операторы (+, — *, /, =)
    Semicolon,             // Символ;
    Comma,                 // Запятая
    LeftParen,             // Круглая скобка (
    RightParen,            // Круглая скобка )
    LeftBracket,           // Квадратная скобка [
    RightBracket,          // Квадратная скобка ]
    EOF,                   // Конец ввода
}

const FUNCTIONS: [&str; 2] = ["print", "write"];

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '/' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == '/' {
                        chars.next();
                        chars.next();
                        while let Some(&ch) = chars.peek() {
                            if ch == '\n' {
                                break;
                            }
                            chars.next();
                        }
                        continue;
                    }
                }
            }
            '0'..='9' => {
                let mut number = String::new();
                let mut is_float = false;

                while let Some(&ch) = chars.peek() {
                    if ch.is_numeric() {
                        number.push(ch);
                        chars.next();
                    } else if ch == '.' {
                        if is_float {
                            panic!("Unexpected '.' in number");
                        }
                        is_float = true;
                        number.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if is_float {
                    tokens.push(Token::Float(number.parse().unwrap()));
                } else {
                    tokens.push(Token::Integer(number.parse().unwrap()));
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        identifier.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if FUNCTIONS.iter().any(|&f| identifier == f) {
                    tokens.push(Token::Functions(identifier));
                } else {
                    tokens.push(Token::Identifier(identifier));
                }
            }
            '"' => {
                let mut string = String::new();
                chars.next();
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        break;
                    } else {
                        string.push(ch);
                        chars.next();
                    }
                }
                tokens.push(Token::Str(string));
                chars.next();
            }
            '+' | '-' | '*' | '/' | '=' | '^' => {
                tokens.push(Token::Operator(c));
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '[' => {
                tokens.push(Token::LeftBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::RightBracket);
                chars.next();
            }
            _ if c.is_whitespace() => {
                chars.next();
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    tokens.push(Token::EOF);
    tokens
}