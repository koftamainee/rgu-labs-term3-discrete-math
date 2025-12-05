use super::ast::{Ast, BinOp};

#[derive(Debug, Clone)]
pub enum Token {
    LParen,
    RParen,
    Op(char),
    Var(String),
}

fn is_var_start(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        match ch {
            '(' | '{' | '[' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' | '}' | ']' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '+' | '&' | '@' | '~' | '>' | '|' | '!' | '-' => {
                tokens.push(Token::Op(ch));
                chars.next();
            }
            c if is_var_start(c) => {
                let mut name = String::new();
                name.push(c);
                chars.next();

                if let Some(&'_') = chars.peek() {
                    name.push('_');
                    chars.next();
                } else {
                    return Err(format!(
                        "Invalid variable '{}': expected underscore after letter",
                        name
                    ));
                }

                let mut has_digits = false;
                while let Some(&nc) = chars.peek() {
                    if nc.is_ascii_digit() {
                        name.push(nc);
                        chars.next();
                        has_digits = true;
                    } else {
                        break;
                    }
                }

                if !has_digits {
                    return Err(format!(
                        "Invalid variable '{}': expected digits after underscore",
                        name
                    ));
                }

                tokens.push(Token::Var(name));
            }
            other => {
                return Err(format!("Unexpected character in input: '{}'", other));
            }
        }
    }
    Ok(tokens)
}

pub fn parse_expr(tokens: &[Token]) -> Result<(Ast, usize), String> {
    parse_at(tokens, 0)
}

fn parse_at(tokens: &[Token], pos: usize) -> Result<(Ast, usize), String> {
    if pos >= tokens.len() {
        return Err("Unexpected end of tokens".to_string());
    }
    match &tokens[pos] {
        Token::Var(name) => Ok((Ast::Var(name.clone()), pos + 1)),
        Token::Op('-') => {
            let (sub, np) = parse_at(tokens, pos + 1)?;
            Ok((Ast::Not(Box::new(sub)), np))
        }
        Token::LParen => {
            let (left, p1) = parse_at(tokens, pos + 1)?;
            if p1 >= tokens.len() {
                return Err("Unexpected end, expected operator after left expr".to_string());
            }
            let op = match &tokens[p1] {
                Token::Op(c) => *c,
                _ => return Err("Expected binary operator after left expression".to_string()),
            };
            if BinOp::from_char(op).is_none() {
                return Err(format!("Unknown binary operator '{}'", op));
            }
            let (right, p2) = parse_at(tokens, p1 + 1)?;
            if p2 >= tokens.len() {
                return Err("Unexpected end, expected ')'".to_string());
            }
            match &tokens[p2] {
                Token::RParen => Ok((
                    Ast::BinOp(
                        BinOp::from_char(op).unwrap(),
                        Box::new(left),
                        Box::new(right),
                    ),
                    p2 + 1,
                )),
                _ => Err("Expected closing ')' after binary expression".to_string()),
            }
        }
        Token::Op(c) => Err(format!("Unexpected operator token '{}'", c)),
        Token::RParen => Err("Unexpected closing parenthesis".to_string()),
    }
}
